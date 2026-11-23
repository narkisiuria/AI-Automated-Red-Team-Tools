#![allow(unused_parens)]
#![allow(unused_must_use)]
#[allow(non_snake_case)]
use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub async fn ask_grok(sys_prompt: &str, user_content: &str) -> Result<String, Box<dyn Error>> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("GROQ_API_KEY").expect("GROQ_API_KEY not set in .env");
    let model: &str = "openai/gpt-oss-20b";

    let call_body = serde_json::json!({
        "model": model,
        "messages": [
            {"role": "system", "content": &sys_prompt},
            {"role": "user", "content": &user_content}
        ],
        "temperature": 0
        });

    let client = if std::env::var("NO_PROXY").is_ok() {
        Client::new()
    } else {
        Client::builder()
            .proxy(reqwest::Proxy::all("socks5h://127.0.0.1:1080")?)
            .build()?
    };
        
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
