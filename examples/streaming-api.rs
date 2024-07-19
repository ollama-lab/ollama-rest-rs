//! This example shows how to build a simple REST API for relaying requests to and
//! streamed responses from Ollama.
//!
//! This example uses [Server-Sent Events (SSE)](https://en.wikipedia.org/wiki/Server-sent_events)
//! for realtime JSON streaming.

use axum::{response::{sse::Event, Sse}, routing::post, Json, Router};
use futures::{Stream, TryStreamExt};
use lazy_static::lazy_static;
use ollama_rest::{errors::Error, models::chat::ChatRequest, Ollama};
use tokio::net::TcpListener;

const HOST_ADDR: &str = "127.0.0.1:9890";

lazy_static! {
    static ref API: Ollama = Ollama::default();
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(chat));

    let listener = TcpListener::bind(HOST_ADDR).await.unwrap();

    print_help_text();

    axum::serve(listener, app).await.unwrap();
}

async fn chat(Json(payload): Json<ChatRequest>) -> Sse<impl Stream<Item = Result<Event, Error>>> {
    Sse::new(
        API.chat_streamed(&payload).await.unwrap()
            .map_ok(|res| Event::default().json_data(res).unwrap()),
    )
}

fn print_help_text() {
    println!("Server listening at {HOST_ADDR}. Press Ctrl+C to exit.");
    println!();
    println!(r#"===
HOW TO USE THIS EXAMPLE:
===
1. Make sure the Ollama server is on!
2. Try calling this API using cURL, or Postman... (whatever you like :D)
For example:
```
curl -X POST http://127.0.0.1:9890/ -H 'Content-Type: application/json' -d '{{
  "model": "llama3",
  "messages": [
    {{
      "role": "user",
      "content": "why is the sky blue?"
    }}
  ]
}}'
```"#);
}
