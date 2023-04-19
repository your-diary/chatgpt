use std::{error::Error, time::Duration};

use reqwest::{
    blocking::{Client, Response},
    header::HeaderMap,
};
use serde::{Deserialize, Serialize};
use serde_json;

use super::config;

/*-------------------------------------*/

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}
impl Message {
    fn new(prompt: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: prompt.to_string(),
        }
    }
}

/*-------------------------------------*/

#[derive(Debug, Deserialize, Serialize)]
struct Req {
    model: String,
    messages: Vec<Message>,
    temperature: f64,
    max_tokens: usize,
}
impl Req {
    fn new(model: &str, messages: Vec<Message>, temperature: f64, max_tokens: usize) -> Self {
        Self {
            model: model.to_string(),
            messages,
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
    message: Message,
}

/*-------------------------------------*/

pub struct ChatGPT {
    client: Client,
    messages: Vec<Message>,
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

        Self {
            client,
            messages: vec![],
        }
    }

    pub fn call(
        &mut self,
        config: &config::Config,
        prompt: &str,
    ) -> Result<String, Box<dyn Error>> {
        let max_tokens = if (prompt.is_ascii()) {
            config.model.max_tokens_en
        } else {
            config.model.max_tokens_ja
        };

        let req_message = Message::new(prompt);
        let mut messages = self.messages.clone();
        messages.push(req_message.clone());

        let req = Req::new(
            &config.model.model,
            messages,
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
            self.messages.push(req_message);
            self.messages
                .push(res.choices.into_iter().next().unwrap().message);
            Ok(self.messages.last().unwrap().content.trim().to_string())
        } else {
            Err(res.text()?.into())
        }
    }
}
