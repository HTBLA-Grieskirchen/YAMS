use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use rustc_hash::FxHashMap;
use yams_core::{
    ResultReport,
    ports::{ObjectStore, ObjectStoreError, ObjectStream, once_stream},
};

#[derive(Clone, Default)]
pub struct FakeObjectStore {
    inner: Arc<Mutex<FxHashMap<String, Vec<u8>>>>,
}

impl FakeObjectStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stored(&self, key: &str) -> Option<Vec<u8>> {
        self.inner
            .lock()
            .expect("fake object store mutex")
            .get(key)
            .cloned()
    }

    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self
            .inner
            .lock()
            .expect("fake object store mutex")
            .keys()
            .cloned()
            .collect();
        keys.sort();
        keys
    }
}

#[async_trait]
impl ObjectStore for FakeObjectStore {
    async fn put(&self, key: &str, bytes: &[u8]) -> ResultReport<(), ObjectStoreError> {
        self.inner
            .lock()
            .expect("fake object store mutex")
            .insert(key.to_string(), bytes.to_vec());
        Ok(())
    }

    async fn get(&self, key: &str) -> ResultReport<Option<ObjectStream>, ObjectStoreError> {
        let bytes = self
            .inner
            .lock()
            .expect("fake object store mutex")
            .get(key)
            .cloned();
        Ok(bytes.map(once_stream))
    }

    async fn delete(&self, key: &str) -> ResultReport<(), ObjectStoreError> {
        self.inner
            .lock()
            .expect("fake object store mutex")
            .remove(key);
        Ok(())
    }
}
