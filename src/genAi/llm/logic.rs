use super::{ollama::OllamaClient, lmStudio::LmStudioClient};
use crate::config::{get_model_provider, LlmModelProvider};

pub enum ClientKind {
    Ollama(OllamaClient),
    LmStudio(LmStudioClient),
}

impl ClientKind {
    pub fn new_default() -> anyhow::Result<Self> {
        match get_model_provider() {
            LlmModelProvider::Ollama => Ok(Self::Ollama(OllamaClient::new("http://localhost:11434", None)?)),
            LlmModelProvider::LmStudio => Ok(Self::LmStudio(LmStudioClient::new("http://localhost:1234", None)?)),
        }
    }

    pub fn list_models(&self) -> anyhow::Result<Vec<String>> {
        match self {
            ClientKind::Ollama(c) => c.list_models(),
            ClientKind::LmStudio(c) => c.list_models(),
        }
    }

    pub fn prompt(&self, model: &str, usr_prompt: &str, sys_prompt: &str) -> anyhow::Result<String> {
        match self {
            ClientKind::Ollama(c) => c.prompt(model, usr_prompt, sys_prompt),
            ClientKind::LmStudio(c) => c.prompt(model, usr_prompt, sys_prompt),
        }
    }

    // pub fn prompt_stream<F: FnMut(&str)>(&self, model: &str, usr_prompt: &str, sys_prompt: &str, on_chunk: F) -> anyhow::Result<()> {
    //     match self {
    //         ClientKind::Ollama(c) => c.prompt_stream(model, usr_prompt, sys_prompt, on_chunk),
    //         ClientKind::LmStudio(c) => c.prompt_stream(model, usr_prompt, sys_prompt, on_chunk),
    //     }
    // }
}

// Convenience helpers
// pub fn stream_prompt<F: FnMut(&str)>(usr_prompt: &str, sys_prompt: &str, mut on_chunk: F) -> anyhow::Result<()> {
//     let model = crate::config::get_model_name();
//     if model.trim().is_empty() { anyhow::bail!("No model configured"); }
//     let client = ClientKind::new_default()?;
//     client.prompt_stream(&model, usr_prompt, sys_prompt, |c| on_chunk(c))
// }
