//#![allow(dead_code)]
mod models;
use models::*;
mod utils;
use utils::*;

use futures_util::stream::StreamExt;
use reqwest::Client;

use serde_json::{from_reader, json};
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, stdin, stdout, BufRead, Write};
use std::path::Path;

static JSON_ENABLED: bool = true;
static COUNT_ENABLED: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    //path naming
    let dir = Path::new("conversations");

    let mut md_filename = ensure_unique_filename(dir.to_str().unwrap())?;
    let json_filename = history_filename(dir.to_str().unwrap())?;

    //debug stuff
    let args: Vec<String> = std::env::args().collect();
    let debug = args.contains(&"--debug".to_string());
    let debug_message = args.contains(&"--debug_message".to_string());

    // Select the provider using a simple condition and function calls.
    let use_openai = args.iter().any(|arg| arg.to_lowercase() == "o");
    let (url, api_key, file_path) = if use_openai {
        setup_openai()
    } else {
        setup_mistral()
    };

    // restore last conversation history
    let restore = args.iter().any(|arg| arg.to_lowercase() == "r");

    let file_contents =
        read_to_string(file_path).expect(&format!("Failed to read the '{}' file", file_path));

    let prompts: Prompts =
        toml::from_str(&file_contents).expect("Could not deserialize the prompts");

    // explain
    if !use_openai {
        println!("Using Mistral, use -- O for OpenAI");
    } else {
        println!("Using OpenAI");
    }
    // select model
    println!("Select model: ");
    print_model_configs(&prompts);
    print!("default [1]: ");

    let mut model_input = String::new();

    stdout().flush().expect("Error: Failed to flush");

    stdin()
        .read_line(&mut model_input)
        .expect("Error: Failed to read line");

    let mut options = Vec::new();

    for option in &prompts.options {
        if option.standard.is_some() {
            options.push((option.model.clone(), option.standard.clone()));
        }
        if option.long.is_some() {
            options.push((option.model.clone(), option.long.clone()));
        }
        if option.code.is_some() {
            options.push((option.model.clone(), option.code.clone()));
        }
    }

    let option_index = model_input.trim().parse::<usize>().unwrap_or(1) - 1; // Default to 1 if parsing fails
    let (model_name, system_prompt) = options
        .get(option_index)
        .ok_or("Invalid option selected")?
        .clone();

    let model_name = model_name.ok_or("Model undefined")?;
    let system_prompt = system_prompt.ok_or("No prompt available")?; // Adjust this line based on your needs

    println!("Model: {}, {}", model_name, model_input.trim());

    let mut messages: Vec<Message>;

    if restore {
        // Open the file
        let file = File::open(json_filename.clone())?;

        // Deserialize the JSON data into a vector of Message structs
        messages = from_reader(file)?;
    } else {
        messages = vec![Message {
            role: "system".to_string(), //system prompt
            content: system_prompt,
        }];
    }

    let mut about_to_quit = false;

    loop {
        let total_tokens: usize = messages
            .iter()
            .map(|msg| tokenize(&msg.content))
            .flatten()
            .count();

        if COUNT_ENABLED {
            println!("-- Total number of tokens: {} --", total_tokens);
        }

        print_colored("You: ").unwrap_or_else(|e| println!("Error printing colored text: {}", e));
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut user_input_single_line = String::new();
        io::stdin().read_line(&mut user_input_single_line).unwrap();

        let user_input: String; // Declare user_input here without initializing
        if user_input_single_line.trim() == "#ML" {
            let mut user_input_multi_line = String::new();
            println!("Multi line mode activated. End input with 'END'.");

            for line in stdin.lock().lines() {
                let line = line.unwrap();
                if line.trim() == "END" {
                    break;
                }
                user_input_multi_line.push_str(&line);
                user_input_multi_line.push('\n');
            }
            user_input = user_input_multi_line.trim_end().to_string();
        } else {
            user_input = user_input_single_line.trim_end().to_string();
        }

        if user_input == "exit" {
            about_to_quit = true;
            // json
            if JSON_ENABLED {
                write_to_json(&json_filename, &messages)?;
            }
            messages.push(Message {
                role: "user".to_string(),
                content: "give this conversation a short name, reply only with: title='name'"
                    .to_string(),
            });
        } else if !about_to_quit {
            messages.push(Message {
                role: "user".to_string(),
                content: user_input.to_string(),
            });
        }

        if user_input == "exit nosave" {
            break;
        }

        if debug_message {
            println!("messages vec:{:?}", messages)
        }

        let body = json!({
            "model": model_name,
            "messages": messages,
            "stream": true
        });

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await;

        let mut response_content = String::new();

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    if !about_to_quit {
                        print_colored("Response: ")
                            .unwrap_or_else(|e| println!("Error printing colored text: {}", e));
                    }

                    let mut stream = resp.bytes_stream();

                    while let Some(chunk) = stream.next().await {
                        match chunk {
                            Ok(bytes) => {
                                let s = String::from_utf8_lossy(&bytes).to_string();
                                let chunks: Vec<&str> = s.split("data: ").collect(); // Split the string by "data: "
                                for chunk in chunks {
                                    if chunk.trim().is_empty() || chunk.trim() == "[DONE]" {
                                        continue; // skip empty strings and the done message
                                    }

                                    if debug {
                                        println!("String before deserialization: {}", chunk);
                                    }

                                    match serde_json::from_str::<Chunk>(chunk) {
                                        Ok(chunk) => {
                                            let content = chunk
                                                .choices
                                                .get(0)
                                                .and_then(|c| Some(c.delta.content.clone()));

                                            let finish_reason = chunk
                                                .choices
                                                .get(0)
                                                .and_then(|c| c.finish_reason.clone());

                                            if let Some(content) = content {
                                                print!("{}", content);
                                                io::stdout().flush().unwrap();
                                                response_content.push_str(&content);
                                                // accumulate the content
                                            }
                                            if let Some(reason) = finish_reason {
                                                if reason == "stop" {}
                                            }
                                        }
                                        Err(e) => {
                                            if debug {
                                                println!(
                                                    "Failed to deserialize chunk: {}, String: {}",
                                                    e, chunk
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => eprintln!("Stream error: {}", e),
                        }
                    }
                    println!("\r");

                    messages.push(Message {
                        role: "assistant".to_string(),
                        content: response_content.clone(),
                    });
                    response_content.clear(); // clear the accumulated content
                } else {
                    eprintln!("Received a failure response: {:?}", resp.status());
                }
            }
            Err(e) => eprintln!("Request error: {}", e),
        }
        if about_to_quit {
            let conversation_md: String = messages
                .iter()
                .map(|msg| format!("- **{}**: {}", msg.role, msg.content))
                .collect::<Vec<String>>()
                .join("\n");

            let mut title = String::new();
            for message in &messages {
                if message.role.to_lowercase() == "assistant"
                    && message.content.to_lowercase().starts_with("title=")
                {
                    let parts: Vec<&str> = message.content.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        title = parts[1].trim().to_string();
                        break;
                    }
                }
            }

            if !title.is_empty() {
                let title_parts: Vec<&str> = title.splitn(2, '\n').collect();
                let sanitized_title = title_parts[0].replace(" ", "_").replace("'", "");
                md_filename = format!("{}--{}", md_filename, sanitized_title);
            }

            md_filename.push_str(".md");

            append_to_markdown(&md_filename, &conversation_md, &model_name)?;

            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
