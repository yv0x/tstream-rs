i lost my cridit card the day before openai billed me so i had to make a wrapper while i waited.
this is my first rust project so it's probably stupid and terrible

quick:
- export $MISTRAL_API_KEY and/or $OPENAI_API_KEY
- git clone
- cargo build
- cargo run (or 'cargo run -- o' for openai, '-- r' for last conversation)


tstream-rs

This repository contains code for a Rust package called "tstream-rs". The package is designed to interact with language models from Mistral and OpenAI and utilize their capabilities. The code is organized into multiple files, including configuration files and the package's manifest file.
Files

    /tstream-rs/mistral_prompts.toml: This is a configuration file for the "mistral" language model. It defines different options and behaviors for three variants of the model: "mistral-medium", "mistral-small", and "mistral-tiny". The variants differ in the level of detail they provide in responses and the inclusion of system prompt text. For example, "mistral-medium" is helpful, concise, and omits mentioning the system prompt text, while "mistral-tiny" provides code snippets without explanations or follow-ups.

    /tstream-rs/openai_prompts.toml: This is another configuration file that sets options for multiple language models. Each model has its own behavior and purpose. Some models are designed to provide concise expert answers on any topic, while others serve as helpful assistants named Sydney. There is also a model specifically tailored for expert programming tasks. Each model's code section is responsible for generating code snippets in response to user requests.

Usage

To use this package, you will need to have Rust and Cargo installed on your system. Follow the Rust installation guide to set up the Rust environment.

Overview tstream-rs/src/main.rs

The main.rs file is the entry point of the Rust program in the tstream-rs repository. It contains the main function, which is the starting point of the program and is executed when the program is run.

Here's a high-level overview of what the main function does:

    Imports: The main function imports various modules and libraries that are needed for the program's functionality.

    API Credentials and Model Selection: The function sets up the API credentials required to interact with the OpenAI language models. It also prompts the user to select a specific model to use.

    File Reading: The program reads a file that contains prompts and options for the conversation with the selected model.

    Interaction with the User: The program prompts the user to provide input and sends the input to the selected model for processing.

    Model Responses: The program receives responses from the model and prints them to the console, allowing the user to see the generated output.

    Conversation History Saving: The program saves the conversation history to a Markdown file, capturing both the user input and the model responses. Optionally, it can also save the conversation history to a JSON file.

    Program Termination: The program continues the interaction with the user until the user inputs "exit", which can be followed by "nosave". At that point, the program exits gracefully.

To use the main.rs file and run the program:

    Make sure you have the necessary dependencies installed as specified in the Cargo.toml file of the repository. You can use cargo build to install the dependencies.

    Configure the API credentials required to interact with the OpenAI language models. This information may be specified in environment variables, configuration files, or through other means.

    Compile and run the program using cargo run or cargo run main.rs.

    Follow the prompts and instructions provided by the program to interact with the selected language model and save the conversation history as desired.

Please note that the specific usage and details may vary depending on the specific implementation and version of the tstream-rs package in the repository.

In the main loop in main.rs, the following special inputs have significance:

    #ML - This activates "multi-line mode" for collecting multiple lines of user input. When this input is received, it prints "Multi line mode activated. End input with 'END'." and uses a for loop to collect multiple lines from stdin until "END" is encountered. This allows for multi-paragraph or multi-line user inputs to be passed to the model.

    exit - This checks if the user has entered the string "exit" to indicate they want to quit the program.

    exit nosave - Adding "nosave" after exit, i.e. the input being "exit nosave", tells the program to quit without saving the conversation history. This is checked via the about_to_quit variable, which if set to true will not write the conversation transcripts to file.
