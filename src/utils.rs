use crate::*;
use chrono::{DateTime, Utc};
use crossterm::{execute, style::Color, style::Print, style::SetForegroundColor};
use std::env;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{stdout, Result, Write};

pub fn append_to_markdown(
    filename: &str,
    conversation_md: &str,
    model_name: &str,
) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(filename)?;

    // Get the current date and time
    let now: DateTime<Utc> = Utc::now();
    // Writing timestamp in a more readable format within the file
    let readable_timestamp = now.format("%A %Y-%m-%d %H:%M (%z)").to_string();
    writeln!(
        &file,
        "Model: {} Timestamp: {}\n---\n{}\n",
        model_name, readable_timestamp, conversation_md
    )?;
    Ok(())
}

pub fn write_to_json(filename: &str, messages: &Vec<Message>) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    // Serialize the conversations to a JSON string
    let json_str = serde_json::to_string_pretty(messages)?;

    // Open the file and write the JSON string to it
    write!(file, "{}", json_str)?;

    Ok(())
}

pub fn ensure_unique_filename(curr_path: &str) -> std::io::Result<String> {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%Y-%m-%d_%H%M").to_string();
    let mut counter = 1;
    let mut base_filename = format!("{}/{}", curr_path, timestamp);

    // Create the directory if it doesn't exist
    let dir = Path::new(curr_path);
    if !dir.exists() {
        create_dir_all(dir)?;
    }
    // Get all files in the directory
    let entries = std::fs::read_dir(curr_path)?;

    // Loop over each entry in the directory
    for entry in entries {
        if let Ok(entry) = entry {
            // Get the filename without extension
            let entry_filename = entry.file_name().to_string_lossy().into_owned();
            let entry_filename_parts: Vec<&str> = entry_filename.split("--").collect();
            let entry_file_timestamp = entry_filename_parts.first().unwrap_or(&"");

            // Split the filename to extract the timestamp part before entering the loop
            let filename_parts: Vec<&str> = base_filename.split('/').collect();
            let filename = filename_parts.last().unwrap_or(&"");
            let timestamp_parts: Vec<&str> = filename.split("--").collect();
            let file_timestamp = timestamp_parts.first().unwrap_or(&"");

            // Compare only the date part of the timestamp
            if entry_file_timestamp == file_timestamp {
                base_filename = format!("{}/{}_{}", curr_path, timestamp, counter);
                counter += 1;
            }
        }
    }
    Ok(base_filename)
}

pub fn history_filename(curr_path: &str) -> std::io::Result<String> {
    let base_filename = format!("{}/_history.json", curr_path);
    Ok(base_filename)
}

pub fn tokenize(input: &str) -> Vec<String> {
    // This is a naive implementation and won't match GPT's tokenization.
    // It's just for demonstration purposes.
    input.split_whitespace().map(|s| s.to_string()).collect()
}

pub fn print_model_configs(prompts: &Prompts) {
    let mut index = 1;
    for model_config in &prompts.options {
        println!(
            "Model: {}",
            model_config.model.as_deref().unwrap_or("undefined")
        );
        if model_config.standard.is_some() {
            println!("[{}] Standard", index);
            index += 1;
        }
        if model_config.long.is_some() {
            println!("[{}] Long", index);
            index += 1;
        }
        if model_config.code.is_some() {
            println!("[{}] Code", index);
            index += 1;
        }
        println!();
    }
}

pub fn print_colored(s: &str) -> Result<()> {
    let mut stdout = stdout();

    // Lighter orange color
    let orange = Color::Rgb {
        r: 255,
        g: 140,
        b: 0,
    };

    execute!(
        stdout,
        SetForegroundColor(orange),
        Print(s),
        SetForegroundColor(Color::Reset)
    )?;

    stdout.flush()?;

    Ok(())
}

pub fn setup_openai() -> (&'static str, String, &'static str) {
    (
        "https://api.openai.com/v1/chat/completions",
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set"),
        "openai_prompts.toml",
    )
}

pub fn setup_mistral() -> (&'static str, String, &'static str) {
    (
        "https://api.mistral.ai/v1/chat/completions",
        env::var("MISTRAL_API_KEY").expect("MISTRAL_API_KEY environment variable not set"),
        "mistral_prompts.toml",
    )
}
