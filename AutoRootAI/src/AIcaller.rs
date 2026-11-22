#![allow(unused_parens)]
#![allow(unused_must_use)]
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::error::Error;

pub async fn ask_grok(sys_prompt: &str, user_content: &str) -> Result<String, Box<dyn Error>> {
    let api_key = std::env::var("GROQ_API_KEY").expect("GROQ_API_KEY not set in .env");
    let model = "llama-3.1-8b-instant";

    let call_body = serde_json::json!({
        "model": "llama-3.1-8b-instant",
        "messages": [
            {"role": "system", "content": &sys_prompt},
            {"role": "user", "content": &user_content}
        ]
        });

    let client = Client::new();
        
    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(api_key) 
        .json(&call_body)
        .send()
        .await?
        .error_for_status()?; 

    let response_json: Value = response.json().await?;

    let reply = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response content")
        .to_string();

    Ok(reply)
}
