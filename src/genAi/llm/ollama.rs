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
            .timeout(None)
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


    pub fn prompt(&self, model: &str, usr_prompt: &str, sys_prompt: &str) -> anyhow::Result<String> {
        #[derive(Serialize)]
        struct ChatReq<'a> { model: &'a str, messages: Vec<Message<'a>>, stream: bool, options: Options }
        #[derive(Serialize)]
        struct Message<'a> { role: &'a str, content: &'a str }
        #[derive(Serialize)]
        struct Options { num_predict: i32 }
        #[derive(Deserialize)]
        struct ChatResp { message: ChatMessage }
        #[derive(Deserialize)]
        struct ChatMessage { content: String }
        let url = format!("{}/api/chat", self.base_url.trim_end_matches('/'));
        let messages = if sys_prompt.trim().is_empty() {
            vec![Message { role: "user", content: usr_prompt }]
        } else {
            vec![
                Message { role: "system", content: sys_prompt },
                Message { role: "user", content: usr_prompt }
            ]
        };
        let options = Options { num_predict: 32768 };
        let body = ChatReq { model, messages, stream: false, options };
        let resp = self.auth_header(self.client.post(url)).json(&body).send()?;
        if !resp.status().is_success() { anyhow::bail!("Ollama chat failed: {}", resp.status()); }
        let data: ChatResp = resp.json()?;
        Ok(data.message.content)
    }

    pub fn prompt_stream<F: FnMut(&str)>(&self, model: &str, usr_prompt: &str, sys_prompt: &str, mut on_chunk: F) -> anyhow::Result<()> {
        #[derive(Serialize)]
        struct ChatReq<'a> { model: &'a str, messages: Vec<Message<'a>>, stream: bool, options: Options }
        #[derive(Serialize)]
        struct Message<'a> { role: &'a str, content: &'a str }
        #[derive(Serialize)]
        struct Options { num_predict: i32 }
        #[derive(Deserialize)]
        struct StreamResp { message: Option<StreamMessage>, done: Option<bool> }
        #[derive(Deserialize)]
        struct StreamMessage { content: Option<String> }
        let url = format!("{}/api/chat", self.base_url.trim_end_matches('/'));
        let messages = if sys_prompt.trim().is_empty() {
            vec![Message { role: "user", content: usr_prompt }]
        } else {
            vec![
                Message { role: "system", content: sys_prompt },
                Message { role: "user", content: usr_prompt }
            ]
        };
        let options = Options { num_predict: 32768 };
        let body = ChatReq { model, messages, stream: true, options };
        let resp = self.auth_header(self.client.post(url)).json(&body).send()?;
        if !resp.status().is_success() { anyhow::bail!("Ollama stream failed: {}", resp.status()); }
        let reader = BufReader::new(resp);
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() { continue; }
            if let Ok(chunk) = serde_json::from_str::<StreamResp>(&line) {
                if let Some(msg) = chunk.message {
                    if let Some(c) = msg.content { on_chunk(&c); }
                }
                if matches!(chunk.done, Some(true)) { break; }
            }
        }
        Ok(())
    }
}
