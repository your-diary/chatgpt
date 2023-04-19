use std::{error::Error, time::Duration};

use reqwest::{
    blocking::{Client, Response},
    header::HeaderMap,
};
use serde::{Deserialize, Serialize};
use serde_json;

use super::config;

/*-------------------------------------*/

#[derive(Debug, Deserialize, Serialize)]
struct Req {
    model: String,
    prompt: String,
    temperature: f64,
    max_tokens: usize,
}
impl Req {
    fn new(model: &str, prompt: &str, temperature: f64, max_tokens: usize) -> Self {
        Self {
            model: model.to_string(),
            prompt: prompt.to_string(),
            temperature,
            max_tokens,
        }
    }
}

/*-------------------------------------*/

#[derive(Debug, Deserialize, Serialize)]
struct Res {
    choices: Vec<Choice>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    text: String,
}

/*-------------------------------------*/

pub struct ChatGPT {
    client: Client,
}
impl ChatGPT {
    pub fn new(config: &config::Config) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", config.api_key).parse().unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Some(Duration::from_millis(config.http.timeout_ms as u64)))
            .build()
            .unwrap();

        Self { client }
    }

    pub fn call(&self, config: &config::Config, prompt: &str) -> Result<String, Box<dyn Error>> {
        let max_tokens = if (prompt.is_ascii()) {
            config.model.max_tokens_en
        } else {
            config.model.max_tokens_ja
        };

        let req = Req::new(
            &config.model.model,
            prompt,
            config.model.temperature,
            max_tokens,
        );

        let res: Response = self
            .client
            .post(&config.http.url)
            .body(serde_json::to_string(&req)?)
            .send()?;

        if res.status().is_success() {
            let text: String = res.text()?;
            let res: Res = serde_json::from_str(&text)?;
            Ok(res
                .choices
                .into_iter()
                .next()
                .unwrap()
                .text
                .trim()
                .to_string())
        } else {
            Err(res.text()?.into())
        }
    }
}
