use std::pin::Pin;

use async_trait::async_trait;
use futures::{Stream, StreamExt, stream};

use crate::ResultReport;

pub type ObjectStream = Pin<Box<dyn Stream<Item = Result<Vec<u8>, ObjectStoreError>> + Send>>;

#[derive(thiserror::Error, Debug)]
pub enum ObjectStoreError {
    #[error("object store operation failed")]
    Operation,
}

#[async_trait]
pub trait ObjectStore: Send + Sync {
    async fn put(&self, key: &str, bytes: &[u8]) -> ResultReport<(), ObjectStoreError>;
    async fn get(&self, key: &str) -> ResultReport<Option<ObjectStream>, ObjectStoreError>;
    async fn delete(&self, key: &str) -> ResultReport<(), ObjectStoreError>;
}

pub async fn collect_object(mut stream: ObjectStream) -> Result<Vec<u8>, ObjectStoreError> {
    let mut out = Vec::new();
    while let Some(chunk) = stream.next().await {
        out.extend(chunk?);
    }
    Ok(out)
}

pub fn once_stream(bytes: Vec<u8>) -> ObjectStream {
    Box::pin(stream::iter(std::iter::once(Ok(bytes))))
}
