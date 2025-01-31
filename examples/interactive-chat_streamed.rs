//! Interactive chat streaming using Stream API
//!
//! Your LLM might mention the "previous talk" in this chat!

use std::io::{BufRead, Write};

use futures::StreamExt;
use ollama_rest::{models::chat::{ChatRequest, Message, Role}, Ollama};
use serde_json::json;
const MODEL_NAME: &str = "llama3.2:1b";

#[tokio::main]
async fn main() {
    // Make sure Ollama serves at 127.0.0.1:11434
    let ollama = Ollama::default();

    let mut messages = Vec::<Message>::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    print!("Loading model... ");
    stdout.flush().unwrap();

    ollama.load_model(MODEL_NAME).await.unwrap();

    println!("done");

    loop {
        let mut prompt = String::new();

        print!("\n>>> ");
        stdout.flush().unwrap();

        // User prompt input
        stdin.lock().read_line(&mut prompt).unwrap();

        // Exit when user typed "/bye"
        if prompt.starts_with("/bye") {
            break;
        }

        messages.push(Message {
            role: Role::User,
            content: prompt,
            images: None,
            tool_calls: None,
        });

        let mut completion = String::new();

        println!();

        // Send conversation to the LLM
        let mut stream = ollama.chat_streamed(
            &serde_json::from_value::<ChatRequest>(json!({
                "model": MODEL_NAME,
                "messages": messages,
            })
        ).unwrap()).await.unwrap();

        while let Some(Ok(res)) = stream.next().await {
            if !res.done {
                if let Some(msg) = &res.message {
                    print!("{}", msg.content);
                    stdout.flush().unwrap();

                    completion.push_str(msg.content.as_str());
                }
            }
        }

        println!();

        messages.push(Message {
            role: Role::Assistant,
            content: completion,
            images: None,
            tool_calls: None,
        });
    }
}
