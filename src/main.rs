use std::env;
use std::io::{self, Write};
use std::process::Command;

use log::error;

use chatgpt::api::ChatGPT;
use chatgpt::config::Config;
use chatgpt::util;

const CONFIG_FILE: &str = "./config.json";

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let config = Config::new(CONFIG_FILE);
    let client = ChatGPT::new(&config);

    let mut buf = String::new();
    loop {
        buf.clear();
        if (config.should_print_prompt) {
            eprint!("> ");
            io::stderr().flush().unwrap();
        }
        match io::stdin().read_line(&mut buf) {
            Ok(0) => {
                eprintln!();
                break;
            }
            Err(e) => {
                error!("{}", e);
                break;
            }
            _ => (),
        }
        if (buf.trim().is_empty()) {
            eprintln!();
            continue;
        }

        let res = (|| {
            for _ in 0..config.http.max_retry {
                match client.call(&config, &buf) {
                    Ok(s) => return s,
                    Err(e) => {
                        if (e.to_string().contains(r#""type": "insufficient_quota""#)) {
                            let _ = Command::new("curl")
                                .args([
                                    &config.discord_url,
                                    "-d",
                                    r#"{"wait": true, "content": "OpenAI API quota exceeded"}"#,
                                    "-H",
                                    "Content-Type: application/json",
                                ])
                                .status();
                            return "QUOTA_ERROR".to_string();
                        } else {
                            error!("{}", e);
                        }
                    }
                }
            }
            "ERROR".to_string()
        })();

        // eprintln!("before = [{}]", res);
        let res = util::prettier(res, config.model.max_tokens_en, config.model.max_tokens_ja);
        // eprintln!(" after = [{}]", res);

        println!("{}", res);
        if (config.should_print_prompt) {
            eprintln!();
        }
    }
}
