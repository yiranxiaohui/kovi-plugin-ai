use kovi::serde_json::json;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use crate::API_KEY;

#[derive(Serialize, Debug, Deserialize)]
pub struct ImageResponse {
    candidates: Vec<Candidate>
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Candidate {
    content: Content
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Content {
    parts: Vec<Part>,
    role: String
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Part {
    text: Option<String>,
    #[serde(rename = "inlineData")]
    inline_data: Option<InlineData>
}

#[derive(Serialize, Debug, Deserialize)]
pub struct InlineData {
    #[serde(rename = "mimeType")]
    mime_type: String,
    data: String
}

pub async fn get_image_base64(image_response: ImageResponse) -> String {
    let candidates = image_response.candidates;
    if candidates.is_empty() {
        return String::new();
    }
    let candidate = candidates.first().unwrap();
    if candidate.content.parts.is_empty() {
        return String::new();
    }
    let part = candidate.content.parts.first().unwrap();
    if part.inline_data.is_none() {
        return String::new();
    }
    let inline_data = part.inline_data.as_ref().unwrap();
    let base64 = &inline_data.data;
    base64.clone()
}

pub async fn gen_image(text: String) -> String {
    let api_key = API_KEY.get();
    if api_key.is_none() {
        return String::new();
    }

    if let Ok(res) = get_image_response(api_key.unwrap().clone(), text).await {
        let base64 = get_image_base64(res).await;
        return format!("base64://{}", base64);
    };
    String::new()
}

pub async fn get_image_response(api_key: String, text: String) -> Result<ImageResponse, Error>{
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-pro-image-preview:generateContent";
    let body = json!({
        "contents": [{
            "parts": [
                {
                    "text": text
                }
            ]
        }]
    });
    let client = Client::new();

    let resp = client
        .post(url)
        .header("x-goog-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let resp = resp.json::<ImageResponse>().await?;

    Ok(resp)
}