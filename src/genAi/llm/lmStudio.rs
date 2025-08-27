use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct LmStudioClient {
    pub base_url: String,
    pub api_key: Option<String>,
    client: reqwest::blocking::Client,
}

impl LmStudioClient {
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

    // LM Studio local API is OpenAI-compatible. We'll implement a subset.
    pub fn list_models(&self) -> anyhow::Result<Vec<String>> {
        #[derive(Deserialize)]
        struct ModelsResp { data: Vec<Model> }
        #[derive(Deserialize)]
        struct Model { id: String }
        let url = format!("{}/v1/models", self.base_url.trim_end_matches('/'));
        let resp = self.auth_header(self.client.get(url)).send()?;
        if !resp.status().is_success() { anyhow::bail!("LM Studio list models failed: {}", resp.status()); }
        let data: ModelsResp = resp.json()?;
        Ok(data.data.into_iter().map(|m| m.id).collect())
    }


    pub fn prompt(&self, model: &str, prompt: &str) -> anyhow::Result<String> {
        #[derive(Serialize)]
        struct ChatReq<'a> { model: &'a str, messages: Vec<Message<'a>>, stream: bool }
        #[derive(Serialize)]
        struct Message<'a> { role: &'a str, content: &'a str }
        #[derive(Deserialize)]
        struct ChatResp { choices: Vec<Choice> }
        #[derive(Deserialize)]
        struct Choice { message: ChoiceMessage }
        #[derive(Deserialize)]
        struct ChoiceMessage { content: String }
        let url = format!("{}/v1/chat/completions", self.base_url.trim_end_matches('/'));
        let body = ChatReq { model, messages: vec![Message { role: "user", content: prompt }], stream: false };
        let resp = self.auth_header(self.client.post(url)).json(&body).send()?;
        if !resp.status().is_success() { anyhow::bail!("LM Studio chat failed: {}", resp.status()); }
        let data: ChatResp = resp.json()?;
        Ok(data.choices.into_iter().next().map(|c| c.message.content).unwrap_or_default())
    }

    pub fn prompt_stream<F: FnMut(&str)>(&self, model: &str, prompt: &str, mut on_chunk: F) -> anyhow::Result<()> {
        #[derive(Serialize)]
        struct ChatReq<'a> { model: &'a str, messages: Vec<Message<'a>>, stream: bool }
        #[derive(Serialize)]
        struct Message<'a> { role: &'a str, content: &'a str }
        #[derive(Deserialize)]
        struct StreamChunk { choices: Vec<DeltaChoice> }
        #[derive(Deserialize)]
        struct DeltaChoice { delta: Delta }
        #[derive(Deserialize)]
        struct Delta { content: Option<String> }
        let url = format!("{}/v1/chat/completions", self.base_url.trim_end_matches('/'));
        let body = ChatReq { model, messages: vec![Message { role: "user", content: prompt }], stream: true };
        let resp = self.auth_header(self.client.post(url)).json(&body).send()?;
        if !resp.status().is_success() { anyhow::bail!("LM Studio chat stream failed: {}", resp.status()); }
        let reader = BufReader::new(resp);
        for line in reader.lines() {
            let line = line?;
            if !line.starts_with("data:") { continue; }
            let payload = line.trim_start_matches("data:").trim();
            if payload == "[DONE]" { break; }
            if let Ok(chunk) = serde_json::from_str::<StreamChunk>(payload) {
                for choice in chunk.choices { if let Some(c) = choice.delta.content { on_chunk(&c); } }
            }
        }
        Ok(())
    }
}
