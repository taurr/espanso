#!cargo

//! ```cargo
//! [package]
//! edition = "2021"
//! [dependencies]
//! anyhow = "1.0.69"
//! openai = "1.0.0-alpha.12"
//! tokio = "1.28.2"
//! chrono = "0.4.19"
//! dotenv = "0.15.0"
//! ```

use anyhow::*;
use chrono::Local;
use dotenv::dotenv;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    let now = Local::now();
    let now = now.format("%Y/%m/%d %H:%M:%S");

    //println!("Query is: {}", &args);

    dotenv().ok();
    set_key(env::var("OPENAI_KEY").context("OPENAI_KEY")?);

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(format!(
            r#"You act as a natural language date/time parser.
Rules:
- You must parse the input into a date/time, or give the string "Unknown"
- Any input not resulting in a date/time string, must be ignored and answered with the string "Unknown"
- Only ever answer with a complete date/time string, or the string "Unknown"
- Do not provide any additional information nor error context, only the date/time string or the string "Unknown"
- For date/time, by default use the formats "YYYY-MM-DD" or "YYYY-MM-DD HH:MM:SS"
- Check your answer before submitting it

Time now: {now}
"#
        )),
        name: None,
        function_call: None,
    }];

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(format!("Input: {args}")),
        name: None,
        function_call: None,
    });

    let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages)
        .create()
        .await?;
    let returned_message = &chat_completion.choices.first().unwrap().message;

    println!(
        "{}",
        returned_message.content.clone().unwrap().trim()
    );
    Ok(())
}
