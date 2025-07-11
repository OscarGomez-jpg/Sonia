use reqwest::Client;
use serde_json::json;

pub async fn get_ai_local_response(prompt: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = "http://localhost:11434/api/generate";
    let body = json!({
        "model": "gemma3",
        "stream": false,
        "num_ctx": 2048,
        "prompt": prompt,
    });

    let resp = client.post(url).json(&body).send().await?;
    let json: serde_json::Value = resp.json().await?;
    if let Some(content) = json.get("response") {
        return Ok(content.as_str().unwrap_or("").to_string());
    } else {
        println!("No content found in response: {:?}", json);
    }
    Ok("".to_string())
}
