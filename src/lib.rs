use std::str::FromStr;

use errors::Error;
use futures_util::StreamExt;
use models::{
    chat::{ChatRequest, ChatResponse},
    create::CreationRequest,
    embeddings::{EmbeddingGenerationRequest, EmbeddingGenerationResponse},
    generate::{GenerationRequest, GenerationResponse},
    model::*,
    Status
};
use reqwest::{Client, ClientBuilder, StatusCode, Url};
#[cfg(feature = "tokio/fs")]
use tokio::fs::File;

pub mod errors;
pub mod models;

macro_rules! streamed_request_wrapper {
    {
        $(#[$attr:meta])*
        $($kw:ident)? fn $func_name:ident($pathname:literal, $req_ty:ty) -> $res_ty:ty$(;)?
    } => {
        $(#[$attr])*
        $($kw)? async fn $func_name<T>(&self, request: &$req_ty, mut on_stream: Option<T>) -> Result<$res_ty, errors::Error>
        where 
            T: FnMut(&$res_ty)
        {
            let res = self.client.post(self.host.join($pathname)?)
                .json(request)
                .send()
                .await?;

            if request.stream.unwrap_or(true) {
                // Handle streamed response
                let mut stream = res.bytes_stream();

                let mut final_res: Option<$res_ty> = None;

                if let Some(ref mut f) = on_stream {
                    while let Some(result) = stream.next().await {
                        let bytes = result?;

                        let cur_res = serde_json::from_slice::<$res_ty>(&bytes)?;
                        f(&cur_res);
                        final_res = Some(cur_res);
                    }
                }

                final_res.ok_or(if on_stream.is_some() { Error::EmptyResponse } else { Error::NoCallback })
            } else {
                // Handle normal response
                Ok(res.json::<$res_ty>().await?)
            }
        }
    };
}

/// The Ollama object that encapsulates everything you need.
///
/// ## Example
///
/// ### Use instance with default config
///
/// ```rust
/// use ollama_rest::Ollama;
///
/// let ollama = Ollama::default();
///
/// // ...
/// ```
///
/// ### Specify URL where Ollama serves
///
/// ```rust
/// use ollama_rest::Ollama;
/// use std::str::FromStr;
///
/// let ollama = Ollama::from_str("http://127.0.0.1:8080").unwrap();
///
/// // ...
/// ```
///
/// ### Provide a Servo URL
///
/// ```rust
/// use ollama_rest::Ollama;
/// use std::str::FromStr;
///
/// let ollama = Ollama::new(url::Url::from_str("http://127.0.0.1:8080").unwrap()).unwrap();
///
/// // ...
/// ```
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

    /// Get host info as a str reference
    pub fn host(&self) -> &str {
        self.host.as_str()
    }

    streamed_request_wrapper! {
        #[doc = "Generate one-shot completion response from one prompt. It can be a completion."]
        pub fn generate("/api/generate", GenerationRequest) -> GenerationResponse
    }

    streamed_request_wrapper! {
        #[doc = "Generate multishot completion response from chat history"]
        pub fn chat("/api/chat", ChatRequest) -> ChatResponse
    }

    streamed_request_wrapper! {
        #[doc = "Create a model"]
        pub fn create("/api/create", CreationRequest) -> Status
    }

    /// Check if blob exists on the server side (not ollama.com)
    ///
    /// ## Parameters
    /// - `digest`: SHA256 digest of the blob
    ///
    /// ## Returns
    /// - `Ok(())`: Blob exists
    /// - `Err(Error::NotExists)`: Blob not exists
    /// - `Err(_)`: Other error
    pub async fn blob_exists(&self, digest: &str) -> Result<(), Error> {
        let status = self.client.head(self.host.join(format!("/api/blobs/sha256:{}", digest).as_str())?)
            .send()
            .await?
            .status();

        match status {
            StatusCode::OK => Ok(()),
            StatusCode::NOT_FOUND => Err(Error::NotExists),
            status => Err(Error::ErrorStatus(status)),
        }
    }

    /// Create a blob
    #[cfg(feature = "blob-creation")]
    pub async fn create_blob(&self, digest: &str, file: File) -> Result<(), Error> {
        let status = self.client.post(self.host.join(format!("/api/blobs/sha256:{}", digest).as_str())?)
            .body(file)
            .send()
            .await?
            .status();

        if let StatusCode::OK = status {
            Ok(())
        } else {
            Err(Error::ErrorStatus(status))
        }
    }

    /// List local models
    pub async fn local_models(&self) -> Result<ModelListResponse, Error> {
        Ok(self.client.get(self.host.join("/api/tags")?)
            .send()
            .await?
            .json::<ModelListResponse>()
            .await?)
    }

    /// Show model information
    pub async fn model(&self, request: &ModelShowRequest) -> Result<ModelShowResponse, Error> {
        Ok(self.client.post(self.host.join("/api/show")?)
            .json(request)
            .send()
            .await?
            .json::<ModelShowResponse>()
            .await?)
    }

    /// Copy a model
    pub async fn copy_model(&self, request: &ModelCopyRequest) -> Result<(), Error> {
        let status = self.client.post(self.host.join("/api/copy")?)
            .json(request)
            .send()
            .await?
            .status();

        match status {
            StatusCode::OK => Ok(()),
            StatusCode::NOT_FOUND => Err(Error::NotExists),
            status => Err(Error::ErrorStatus(status)),
        }
    }

    /// Delete a model
    pub async fn delete_model(&self, request: &ModelDeletionRequest) -> Result<(), Error> {
        let status = self.client.delete(self.host.join("/api/delete")?)
            .json(request)
            .send()
            .await?
            .status();

        match status {
            StatusCode::OK => Ok(()),
            StatusCode::NOT_FOUND => Err(Error::NotExists),
            status => Err(Error::ErrorStatus(status)),
        }
    }

    /// Pull a model
    pub async fn pull_model<T>(&self, request: &ModelSyncRequest, mut on_stream: Option<T>) -> Result<Status, Error>
    where 
        T: FnMut(&ModelPullStatusKind),
    {
        let res = self.client.post(self.host.join("/api/pull")?)
            .json(request)
            .send()
            .await?;

        if request.stream.unwrap_or(true) {
            // Handle streamed response
            let mut stream = res.bytes_stream();

            let mut final_res: Option<Status> = None;

            if let Some(ref mut f) = on_stream {
                while let Some(result) = stream.next().await {
                    let bytes = result?;

                    let cur_res = serde_json::from_slice::<ModelPullStatusKind>(&bytes)?;
                    f(&cur_res);

                    if let ModelPullStatusKind::Message(cur_res) = cur_res {
                        final_res = Some(cur_res);
                    }
                }
            }

            final_res.ok_or(if on_stream.is_some() { Error::EmptyResponse } else { Error::NoCallback })
        } else {
            // Handle normal response
            Ok(res.json::<Status>().await?)
        }
    }

    /// Push a model
    pub async fn push_model<T>(&self, request: &ModelSyncRequest, mut on_stream: Option<T>) -> Result<Status, Error>
    where 
        T: FnMut(&ModelPushStatusKind),
    {
        let res = self.client.post(self.host.join("/api/push")?)
            .json(request)
            .send()
            .await?;

        if request.stream.unwrap_or(true) {
            // Handle streamed response
            let mut stream = res.bytes_stream();

            let mut final_res: Option<Status> = None;

            if let Some(ref mut f) = on_stream {
                while let Some(result) = stream.next().await {
                    let bytes = result?;

                    let cur_res = serde_json::from_slice::<ModelPushStatusKind>(&bytes)?;
                    f(&cur_res);

                    if let ModelPushStatusKind::Message(cur_res) = cur_res {
                        final_res = Some(cur_res);
                    }
                }
            }

            final_res.ok_or(if on_stream.is_some() { Error::EmptyResponse } else { Error::NoCallback })
        } else {
            // Handle normal response
            Ok(res.json::<Status>().await?)
        }
    }

    /// Generate embeddings
    pub async fn generate_embeddings(&self, request: &EmbeddingGenerationRequest) -> Result<EmbeddingGenerationResponse, Error> {
        Ok(self.client.post(self.host.join("/api/embeddings")?)
            .json(request)
            .send()
            .await?
            .json::<EmbeddingGenerationResponse>()
            .await?)
    }

    /// List running models
    pub async fn running_models(&self) -> Result<RunningModelResponse, Error> {
        Ok(self.client.get(self.host.join("/api/ps")?)
            .send()
            .await?
            .json::<RunningModelResponse>()
            .await?)
    }
}

impl FromStr for Ollama {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(Url::from_str(s)?)
    }
}

impl Default for Ollama {
    fn default() -> Self {
        Self::from_str("http://127.0.0.1:11434").unwrap()
    }
}
