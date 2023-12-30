use serde::{Deserialize, Serialize};
use core::fmt::Debug;

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Choices {
    //index: u32,
    pub delta: Delta,
    pub finish_reason: Option<String>, // Adjusted to handle null
}
#[derive(Debug, Deserialize)]
pub struct Chunk {
    pub choices: Vec<Choices>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct Conversation {
    pub role: String,
    pub content: String,
}

// Define structure for prompts file content
#[derive(Debug, Deserialize)]
pub struct Prompts {
    pub options: Vec<ModelConfig>,
}
#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    pub model: Option<String>,
    pub long: Option<String>,
    pub code: Option<String>,
    pub standard: Option<String>,
}
