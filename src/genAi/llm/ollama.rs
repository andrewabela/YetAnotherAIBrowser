use serde::{Deserialize, Serialize};

use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct OllamaClient {
    pub base_url: String,
    pub api_key: Option<String>,
    client: reqwest::blocking::Client,
}

impl OllamaClient {
    pub fn new<S: Into<String>>(base_url: S, api_key: Option<String>) -> anyhow::Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("yetanotheraibrowser/0.1")
            .build()?;
        Ok(Self { base_url: base_url.into(), api_key, client })
    }

    fn auth_header(&self, req: reqwest::blocking::RequestBuilder) -> reqwest::blocking::RequestBuilder {
        if let Some(key) = &self.api_key {
            req.header("Authorization", format!("Bearer {}", key))
        } else { req }
    }

    pub fn list_models(&self) -> anyhow::Result<Vec<String>> {
        #[derive(Deserialize)]
    struct ModelsResp { models: Vec<Model> }
        #[derive(Deserialize)]
    struct Model { name: String }
        let url = format!("{}/api/tags", self.base_url.trim_end_matches('/'));
        let resp = self.auth_header(self.client.get(url)).send()?;
        if !resp.status().is_success() { anyhow::bail!("Ollama list models failed: {}", resp.status()); }
        let data: ModelsResp = resp.json()?;
        Ok(data.models.into_iter().map(|m| m.name).collect())
    }


    pub fn prompt(&self, model: &str, prompt: &str) -> anyhow::Result<String> {
        #[derive(Serialize)]
        struct GenReq<'a> { model: &'a str, prompt: &'a str, stream: bool }
        #[derive(Deserialize)]
        struct GenResp { response: String }
        let url = format!("{}/api/generate", self.base_url.trim_end_matches('/'));
        let resp = self.auth_header(self.client.post(url))
            .json(&GenReq { model, prompt, stream: false })
            .send()?;
        if !resp.status().is_success() { anyhow::bail!("Ollama generate failed: {}", resp.status()); }
        let data: GenResp = resp.json()?;
        Ok(data.response)
    }

    pub fn prompt_stream<F: FnMut(&str)>(&self, model: &str, prompt: &str, mut on_chunk: F) -> anyhow::Result<()> {
        #[derive(Serialize)]
        struct GenReq<'a> { model: &'a str, prompt: &'a str, stream: bool }
        #[derive(Deserialize)]
        struct StreamResp { response: Option<String>, done: Option<bool> }
        let url = format!("{}/api/generate", self.base_url.trim_end_matches('/'));
        let resp = self.auth_header(self.client.post(url))
            .json(&GenReq { model, prompt, stream: true })
            .send()?;
        if !resp.status().is_success() { anyhow::bail!("Ollama stream failed: {}", resp.status()); }
        let reader = BufReader::new(resp);
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() { continue; }
            if let Ok(chunk) = serde_json::from_str::<StreamResp>(&line) {
                if let Some(r) = chunk.response.as_ref() { on_chunk(r); }
                if matches!(chunk.done, Some(true)) { break; }
            }
        }
        Ok(())
    }
}
