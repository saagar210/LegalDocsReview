mod provider;
mod types;
pub(crate) mod prompts;
mod ollama;
mod claude;
mod openai;

pub(crate) use provider::AiProvider;
pub(crate) use types::*;
pub(crate) use ollama::OllamaProvider;
pub(crate) use claude::ClaudeProvider;
pub(crate) use openai::OpenAiProvider;
