# Ollama-REST.rs

Asynchronous Rust bindings of Ollama REST API,
using [reqwest](https://github.com/seanmonstar/reqwest),
[tokio](https://tokio.rs),
[serde](https://serde.rs/),
and [chrono](https://github.com/chronotope/chrono).

## At a glance

> See [source](./examples/one-time-chat.rs) of this example.

```rust
use std::io::Write;

use ollama_rest::{models::generate::{GenerationRequest, GenerationResponse}, Ollama};
use serde_json::json;

// By default checking Ollama at 127.0.0.1:11434
let ollama = Ollama::default();

let request = serde_json::from_value::<GenerationRequest>(json!({
    "model": "llama3",
    "prompt": "Why is the sky blue?",
})).unwrap();

let final_res = ollama.generate(
    &request,
    // Handle streamed response
    Some(|res: &GenerationResponse| {
        if !res.done {
            print!("{}", res.response);
            // Flush stdout for each word to allow realtime output
            std::io::stdout().flush().unwrap();
        }
    })
).await.unwrap();

println!("\n\nFinal response:");
println!("{final_res:?}");
```

Or, make your own chatbot interface! See [this example](./examples/interactive-chat.rs).
