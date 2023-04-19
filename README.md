# 1. About

An interactive client for [OpenAI API](https://openai.com/blog/openai-api), implemented in Rust.

[`/v1/char/completions`](https://platform.openai.com/docs/api-reference/chat/create) is called under the hood.

<sub>(In the initial commit `f57ed16`, a client for [`/v1/completions`](https://platform.openai.com/docs/api-reference/completions) was implemented.)</sub>

# 2. Configurations

`./config.json` is used as a configuration file.

`discord_url` field is used to send a notification to Discord when a fatal error (e.g. `insufficient_quota`) occurs.

Example:
```json
{
    "api_key": "abcde",
    "discord_url": "https://discord.com/api/webhooks/12345/abcde",
    "should_print_prompt": false,
    "http": {
        "url": "https://api.openai.com/v1/chat/completions",
        "timeout_ms": 12000,
        "max_retry": 2
    },
    "model": {
        "model": "gpt-4",
        "temperature": 0.9,
        "max_tokens_en": 30,
        "max_tokens_ja": 140
    }
}
```

# 3. Usage

```bash
$ cargo run --release
```

# 4. References

- [*API Reference*](https://platform.openai.com/docs/api-reference/chat)

<!-- vim: set spell: -->

