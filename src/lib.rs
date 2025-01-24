//! Asynchronous Rust bindings of Ollama REST API.

use std::str::FromStr;

use errors::Error;
use futures::{Stream, StreamExt};
use models::{
    chat::{ChatRequest, ChatResponse},
    create::CreationRequest,
    embeddings::{EmbeddingGenerationRequest, EmbeddingGenerationResponse},
    generate::{GenerationRequest, GenerationResponse},
    model::*,
    Status
};
use reqwest::{Client, ClientBuilder, StatusCode, Url};
use tokio::fs::File;

pub mod errors;
pub mod models;
pub mod prelude;

// Re-exports
pub use chrono;
pub use futures;

macro_rules! streamed_request_wrapper {
    {
        $(
            $(#[$attr:meta])*
            $($kw:ident)? fn $func_name:ident($pathname:literal, $req_ty:ty) -> $res_ty:ty
            $(=> streamed as [
                $(#[$attr2:meta])*
                $($kw2:ident)? fn $streamed_func_name:ident
            ])?
        );*
        $(;)?
    } => {
        $(
            $(#[$attr])*
            $($kw)? async fn $func_name<T>(&self, request: &$req_ty, mut on_stream: Option<T>) -> Result<$res_ty, Error>
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

            $(
                $(#[$attr2])*
                $($kw2)? async fn $streamed_func_name(&self, request: &$req_ty) -> Result<impl Stream<Item = Result<$res_ty, Error>>, Error> {
                    if !request.stream.unwrap_or(true) {
                        return Err(Error::StreamingOff);
                    }

                    let res = self.client.post(self.host.join($pathname)?)
                        .json(request)
                        .send()
                        .await?;

                    Ok(
                        res.bytes_stream()
                        .map(|result| {
                            result.map_err(|err| Error::from(err))
                                .and_then(|ref bytes| {
                                    serde_json::from_slice::<$res_ty>(bytes)
                                        .map_err(|err| Error::from(err))
                                })
                        })
                    )
                }

            )?
        )*
    };

    {
        @nocheck
        $(
            $(#[$attr:meta])*
            $($kw:ident)? fn $func_name:ident($pathname:literal, $req_ty:ty) -> $res_ty:ty
            $(=> streamed as [
                $(#[$attr2:meta])*
                $($kw2:ident)? fn $streamed_func_name:ident
            ])?
        );*
        $(;)?
    } => {
        $(
            $(#[$attr])*
            $($kw)? async fn $func_name<T>(&self, request: &$req_ty, mut on_stream: Option<T>) -> Result<$res_ty, Error>
            where 
                T: FnMut(&$res_ty)
            {
                let res = self.client.post(self.host.join($pathname)?)
                    .json(request)
                    .send()
                    .await?;

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
            }

            $(
                $(#[$attr2])*
                $($kw2)? async fn $streamed_func_name(&self, request: &$req_ty) -> Result<impl Stream<Item = Result<$res_ty, Error>>, Error> {
                    let res = self.client.post(self.host.join($pathname)?)
                        .json(request)
                        .send()
                        .await?;

                    Ok(
                        res.bytes_stream()
                        .map(|result| {
                            result.map_err(|err| Error::from(err))
                                .and_then(|ref bytes| {
                                    serde_json::from_slice::<$res_ty>(bytes)
                                        .map_err(|err| Error::from(err))
                                })
                        })
                    )
                }

            )?
        )*
    };
}

/// The Ollama instance encapsulating everything you need.
///
/// ## Examples
///
/// ### Use instance with default configuration
///
/// ```rust
/// use ollama_rest::Ollama;
///
/// let ollama = Ollama::default();
///
/// // ...
/// ```
///
/// ### Specify a URL where Ollama serves
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
#[derive(Clone)]
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
        #[doc = "Generate completion response for one single prompt (Callback API)"]
        pub fn generate("/api/generate", GenerationRequest) -> GenerationResponse
            => streamed as [
                #[doc = "Generate completion response for one single prompt (Stream API)"]
                pub fn generate_streamed
            ];

        #[doc = "Generate completion response from chat history (Callback API)"]
        pub fn chat("/api/chat", ChatRequest) -> ChatResponse
            => streamed as [
                #[doc = "Generate completion response from chat history (Stream API)"]
                pub fn chat_streamed
            ];

        #[doc = "create a model (Callback API)"]
        pub fn create("/api/create", CreationRequest) -> Status
            => streamed as [
                #[doc = "create a model (Stream API)"]
                pub fn create_streamed
            ];
    }

    /// Load a model
    ///
    /// It calls `/api/generate` with no prompt, which makes Ollama to load
    /// the model.
    pub async fn load_model(&self, model: &str) -> Result<GenerationResponse, Error> {
        Ok(self.client.post(self.host.join("/api/generate")?)
            .json(&serde_json::json!({ "model": model }))
            .send()
            .await?
            .json::<GenerationResponse>()
            .await?)
    }

    /// Check if blob exists on the server side (not ollama.com)
    ///
    /// ## Parameters
    /// - `digest`: SHA256 digest of the blob
    ///     **Be aware**: Currently the digest will be directly appended into the URL
    ///     without sanitization, please don't expose digest input to end user side.
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
    ///
    /// ## Parameters
    /// - `digest`: SHA256 digest of the blob
    ///     **Be aware**: Currently the digest will be directly appended into the URL
    ///     without sanitization, please don't expose digest input to end user side.
    /// - `file`: Tokio File instance
    ///
    /// ## Returns
    /// - `Ok(())`: Blob created
    /// - `Err(_)`: Error occurred
    pub async fn create_blob(&self, digest: &str, file: File) -> Result<(), Error> {
        let status = self.client.post(self.host.join(format!("/api/blobs/sha256:{}", digest).as_str())?)
            .body(file)
            .send()
            .await?
            .status();

        if let StatusCode::CREATED = status {
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
    ///
    /// ## Returns
    /// - `Ok(())`: Model copied
    /// - `Err(Error::NotExists)`: Source model not exists
    /// - `Err(_)`: Other error
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
    ///
    /// ## Returns
    /// - `Ok(())`: Model deleted
    /// - `Err(Error::NotExists)`: Target model not exists
    /// - `Err(_)`: Other error
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

    streamed_request_wrapper! {
        @nocheck

        #[doc = "Pull a model (Callback API)"]
        pub fn pull_model("/api/pull", ModelSyncRequest) -> ModelPullStatus
            => streamed as [
                #[doc = "Pull a model (Stream API)"]
                pub fn pull_model_streamed
            ];

        #[doc = "Push a model (Callback API)"]
        pub fn push_model("/api/push", ModelSyncRequest) -> ModelPushStatus
            => streamed as [
                #[doc = "Push a model (Stream API)"]
                pub fn push_model_streamed
            ];
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
