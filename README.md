# ollama-rest.rs

Asynchronous Rust bindings of [Ollama REST API](https://github.com/ollama/ollama/blob/main/docs/api.md),
using [reqwest](https://github.com/seanmonstar/reqwest),
[tokio](https://tokio.rs),
[serde](https://serde.rs/),
and [chrono](https://github.com/chronotope/chrono).

## Install

```bash
cargo add ollama-rest
```

## Features

|    name        |     status      |
|----------------|-----------------|
| Completion     | Supported ✅    |
| Embedding      | Supported ✅    |
| Model creation | Supported ✅    |
| Model deletion | Supported ✅    |
| Model pulling  | Supported ✅    |
| Model copying  | Supported ✅    |
| Local models   | Supported ✅    |
| Running models | Supported ✅    |
| Model pushing  | Experimental 🧪 |
| Tools          | Experimental 🧪 |

## At a glance

> See [source](./examples/generate_streamed.rs) of this example.

```rust
use std::io::Write;

use ollama_rest::{models::generate::{GenerationRequest, GenerationResponse}, Ollama};
use serde_json::json;

// By default checking Ollama at 127.0.0.1:11434
let ollama = Ollama::default();

let request = serde_json::from_value::<GenerationRequest>(json!({
    "model": "llama3.2:1b",
    "prompt": "Why is the sky blue?",
})).unwrap();

let mut stream = ollama.generate_streamed(&request).await.unwrap();

while let Some(Ok(res)) = stream.next().await {
    if !res.done {
        print!("{}", res.response);
        // Flush stdout for each word to allow realtime output
        std::io::stdout().flush().unwrap();
    }
}

println!();
```

Or, make your own chatbot interface! See [this example](./examples/interactive-chat_streamed.rs) (CLI) and [this example](./examples/streaming-relay.rs) (REST API).

