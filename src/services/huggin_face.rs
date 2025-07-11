use reqwest::Client;
use serde_json::json;

pub async fn get_ai_response(prompt: &str, api_token: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = "https://router.huggingface.co/hf-inference/models/HuggingFaceTB/SmolLM3-3B/v1/chat/completions";
    let body = json!({
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "model": "HuggingFaceTB/SmolLM3-3B",
        "stream": false
    });

    let resp = client
        .post(url)
        .bearer_auth(api_token)
        .json(&body)
        .send()
        .await?;

    let json: serde_json::Value = resp.json().await?;
    if let Some(choices) = json.get("choices").and_then(|v| v.as_array()) {
        if let Some(choice) = choices.first() {
            if let Some(message) = choice.get("message") {
                if let Some(content) = message.get("content") {
                    return Ok(content.as_str().unwrap_or("").to_string());
                }
            }
        }
    }
    Ok("".to_string())
}
