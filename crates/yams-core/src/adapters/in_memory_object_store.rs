use std::sync::Mutex;

use async_trait::async_trait;
use rustc_hash::FxHashMap;
use tracing::debug;

use crate::ResultReport;
use crate::ports::{ObjectStore, ObjectStoreError, ObjectStream, once_stream};
use error_stack::Report;

#[derive(Default)]
pub struct InMemoryObjectStore {
    inner: Mutex<FxHashMap<String, Vec<u8>>>,
}

impl InMemoryObjectStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ObjectStore for InMemoryObjectStore {
    async fn put(&self, key: &str, bytes: &[u8]) -> ResultReport<(), ObjectStoreError> {
        self.inner
            .lock()
            .expect("in-memory object store mutex")
            .insert(key.to_string(), bytes.to_vec());
        debug!(key, bytes_len = bytes.len(), "in-memory object store put");
        Ok(())
    }

    async fn get(&self, key: &str) -> ResultReport<Option<ObjectStream>, ObjectStoreError> {
        let bytes = self
            .inner
            .lock()
            .expect("in-memory object store mutex")
            .get(key)
            .cloned();
        debug!(key, found = bytes.is_some(), "in-memory object store get");
        Ok(bytes.map(once_stream))
    }

    async fn delete(&self, key: &str) -> ResultReport<(), ObjectStoreError> {
        match self
            .inner
            .lock()
            .expect("in-memory object store mutex")
            .remove(key)
        {
            Some(_) => {
                debug!(key, "in-memory object store delete");
                Ok(())
            }
            None => Err(Report::new(ObjectStoreError::AlreadyDeleted)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::collect_object;

    #[pollster::test]
    async fn put_then_get_roundtrip() {
        let store = InMemoryObjectStore::new();
        store.put("a/b", b"hello").await.unwrap();
        let stream = store.get("a/b").await.unwrap().expect("stored");
        assert_eq!(collect_object(stream).await.unwrap(), b"hello");
    }

    #[pollster::test]
    async fn get_missing_is_none() {
        let store = InMemoryObjectStore::new();
        assert!(store.get("missing").await.unwrap().is_none());
    }

    #[pollster::test]
    async fn put_overwrites() {
        let store = InMemoryObjectStore::new();
        store.put("k", b"one").await.unwrap();
        store.put("k", b"two").await.unwrap();
        let stream = store.get("k").await.unwrap().unwrap();
        assert_eq!(collect_object(stream).await.unwrap(), b"two");
    }

    #[pollster::test]
    async fn delete_removes_object() {
        let store = InMemoryObjectStore::new();
        store.put("k", b"gone").await.unwrap();
        store.delete("k").await.unwrap();
        assert!(store.get("k").await.unwrap().is_none());
    }

    #[pollster::test]
    async fn delete_missing_is_already_deleted() {
        let store = InMemoryObjectStore::new();
        let err = store.delete("missing").await.unwrap_err();
        assert!(matches!(
            err.current_context(),
            ObjectStoreError::AlreadyDeleted
        ));
    }

    #[pollster::test]
    async fn ensure_deleted_swallows_already_deleted() {
        let store = InMemoryObjectStore::new();
        store.ensure_deleted("missing").await.unwrap();
    }
}
