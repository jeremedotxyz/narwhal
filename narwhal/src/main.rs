use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use chrono::Utc;
use anyhow::Result;

#[derive(Serialize, Deserialize)]
struct Prompt { id: u32, content: String }

#[derive(Serialize, Deserialize, Debug)]
struct ModelResponse { choices: Vec<Choice> }

#[derive(Serialize, Deserialize, Debug)]
struct Choice { message: Message }

#[derive(Serialize, Deserialize, Debug)]
struct Message { content: String }

struct NarwhalHarness { api_key: String, endpoint: String, model: String }

impl NarwhalHarness {
    fn new(api_key: String, endpoint: String, model: String) -> Self { Self { api_key, endpoint, model } }
    async fn evaluate(&self, prompt: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let res = client.post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({"model": self.model, "messages": [{"role": "user", "content": prompt}]}))
            .send().await?.json::<ModelResponse>().await?;
        Ok(res.choices[0].message.content.clone())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Narwhal Harness Initialized...");
    Ok(())
}
