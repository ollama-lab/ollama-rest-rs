use std::str::FromStr;

use errors::Error;
use models::{chat::{ChatRequest, ChatResponse}, generate::{GenerateRequest, GenerateResponse}};
use reqwest::{Client, ClientBuilder, Url};

pub mod errors;
pub mod models;

/// The Ollama object that encapsulates everything you need.
pub struct Ollama {
    host: Url,
    client: Client,
}

impl Ollama {
    #[must_use]
    pub fn new(host: Url) -> Result<Self, Error> {
        Ok(Self {
            host,
            client: ClientBuilder::new()
                .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
                .build()?,
        })
    }

    /// Get host info
    pub fn host(&self) -> &str {
        self.host.as_str()
    }

    /// Generate response from one prompt
    pub async fn generate<T>(&self, request: &GenerateRequest, mut on_stream: Option<T>) -> Result<GenerateResponse, Error>
    where 
        T: FnMut(GenerateResponse)
    {
        let res = self.client.post(self.host.join("/api/generate")?)
            .json(request)
            .send()
            .await?;

        if request.stream.unwrap_or(true) {
            let mut stream = res.bytes_stream();

            let mut final_res: Option<GenerateResponse> = None;

            if let Some(f) = on_stream {
                while let Some(item) = stream.next().await {
                    final_res = Some(item);
                    f(item);
                }
            }

            todo!();
            final_res.ok_or(Err(Error::Event))
        } else {
            res.json::<GenerateResponse>()
                .await?;
        }
    }

    /// Generate next response from chat memory
    pub async fn chat(&self, request: &ChatRequest) -> Result<ChatResponse, Error> {
        Ok(
            self.client.post(self.host.join("/api/chat")?)
                .json(request)
                .send()
                .await?
                .json::<ChatResponse>()
                .await?
        )
    }
}

impl FromStr for Ollama {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(Url::from_str(s).or_else(|err| Err(Error::UrlParsing(err)))?)
    }
}

impl Default for Ollama {
    fn default() -> Self {
        Self::from_str("http://127.0.0.1:11434").unwrap()
    }
}
