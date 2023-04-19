use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub api_key: String,
    pub discord_url: String,
    pub should_print_prompt: bool,
    pub http: Http,
    pub model: Model,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Http {
    pub url: String,
    pub timeout_ms: usize,
    pub max_retry: usize,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Model {
    pub model: String,
    pub temperature: f64,
    pub max_tokens_en: usize,
    pub max_tokens_ja: usize,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CoeFont {
    pub enabled: bool,
    pub binary_path: String,
}

impl Config {
    pub fn new(config_file: &str) -> Self {
        let json_string: String = {
            let file = File::open(config_file).unwrap();
            let comment_regex = Regex::new(r#"^\s*#.*"#).unwrap();
            BufReader::new(file)
                .lines()
                .filter(|l| !comment_regex.is_match(l.as_ref().unwrap()))
                .map(|l| l.unwrap())
                .collect::<Vec<String>>()
                .join("\n")
        };
        serde_json::from_str(&json_string).unwrap()
    }
}
