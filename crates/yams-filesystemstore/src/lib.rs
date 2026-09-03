//! Filesystem adapter for the YAMS `ObjectStore` port.

use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use async_trait::async_trait;
use error_stack::Report;
use futures::Stream;
use tempdir::TempDir;
use yams_core::{
    ErrorReportExt, ResultReport,
    ports::{ObjectStore, ObjectStoreError, ObjectStream},
};

const CHUNK: usize = 64 * 1024;

pub struct FileSystemObjectStore {
    root: PathBuf,
    _temp: Option<Arc<TempDir>>,
}

impl FileSystemObjectStore {
    pub fn new(dir: impl Into<PathBuf>) -> ResultReport<Self, ObjectStoreError> {
        let root = dir.into();
        fs::create_dir_all(&root).contextualize(ObjectStoreError::Operation)?;
        Ok(Self { root, _temp: None })
    }

    pub fn in_temp_dir() -> ResultReport<Self, ObjectStoreError> {
        let temp = TempDir::new("yams-object-store").contextualize(ObjectStoreError::Operation)?;
        let root = temp.path().to_path_buf();
        Ok(Self {
            root,
            _temp: Some(Arc::new(temp)),
        })
    }

    fn path_for(&self, key: &str) -> Result<PathBuf, ObjectStoreError> {
        key_to_path(&self.root, key)
    }
}

fn key_to_path(root: &Path, key: &str) -> Result<PathBuf, ObjectStoreError> {
    if key.is_empty() || key.starts_with('/') {
        return Err(ObjectStoreError::Operation);
    }
    let mut path = root.to_path_buf();
    for segment in key.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." {
            return Err(ObjectStoreError::Operation);
        }
        if Path::new(segment).is_absolute() {
            return Err(ObjectStoreError::Operation);
        }
        path.push(segment);
    }
    Ok(path)
}

struct FileStream {
    file: File,
}

impl Stream for FileStream {
    type Item = Result<Vec<u8>, ObjectStoreError>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = vec![0; CHUNK];
        match self.file.read(&mut buf) {
            Ok(0) => Poll::Ready(None),
            Ok(n) => {
                buf.truncate(n);
                Poll::Ready(Some(Ok(buf)))
            }
            Err(_) => Poll::Ready(Some(Err(ObjectStoreError::Operation))),
        }
    }
}

#[async_trait]
impl ObjectStore for FileSystemObjectStore {
    async fn put(&self, key: &str, bytes: &[u8]) -> ResultReport<(), ObjectStoreError> {
        let path = self.path_for(key)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).contextualize(ObjectStoreError::Operation)?;
        }
        fs::write(path, bytes).contextualize(ObjectStoreError::Operation)?;
        Ok(())
    }

    async fn get(&self, key: &str) -> ResultReport<Option<ObjectStream>, ObjectStoreError> {
        let path = self.path_for(key)?;
        match File::open(path) {
            Ok(file) => Ok(Some(Box::pin(FileStream { file }))),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err).contextualize(ObjectStoreError::Operation),
        }
    }

    async fn delete(&self, key: &str) -> ResultReport<(), ObjectStoreError> {
        let path = self.path_for(key)?;
        match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                Err(Report::new(ObjectStoreError::AlreadyDeleted))
            }
            Err(err) => Err(err).contextualize(ObjectStoreError::Operation),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yams_core::ports::collect_object;

    #[pollster::test]
    async fn put_get_roundtrip() {
        let store = FileSystemObjectStore::in_temp_dir().unwrap();
        store.put("rechnungen/a.pdf", b"pdf-bytes").await.unwrap();
        let stream = store.get("rechnungen/a.pdf").await.unwrap().unwrap();
        assert_eq!(collect_object(stream).await.unwrap(), b"pdf-bytes");
    }

    #[pollster::test]
    async fn missing_is_none() {
        let store = FileSystemObjectStore::in_temp_dir().unwrap();
        assert!(store.get("nope").await.unwrap().is_none());
    }

    #[pollster::test]
    async fn rejects_parent_segment() {
        let store = FileSystemObjectStore::in_temp_dir().unwrap();
        assert!(store.put("../escape", b"x").await.is_err());
        assert!(store.put("a/../b", b"x").await.is_err());
        assert!(store.get("/abs").await.is_err());
        assert!(store.put("", b"x").await.is_err());
    }

    #[pollster::test]
    async fn overwrites_existing() {
        let store = FileSystemObjectStore::in_temp_dir().unwrap();
        store.put("k", b"one").await.unwrap();
        store.put("k", b"two").await.unwrap();
        let stream = store.get("k").await.unwrap().unwrap();
        assert_eq!(collect_object(stream).await.unwrap(), b"two");
    }

    #[pollster::test]
    async fn delete_removes_object() {
        let store = FileSystemObjectStore::in_temp_dir().unwrap();
        store.put("rechnungen/a.pdf", b"pdf").await.unwrap();
        store.delete("rechnungen/a.pdf").await.unwrap();
        assert!(store.get("rechnungen/a.pdf").await.unwrap().is_none());
    }

    #[pollster::test]
    async fn delete_missing_is_already_deleted() {
        let store = FileSystemObjectStore::in_temp_dir().unwrap();
        let err = store.delete("rechnungen/missing.pdf").await.unwrap_err();
        assert!(matches!(
            err.current_context(),
            ObjectStoreError::AlreadyDeleted
        ));
    }

    #[pollster::test]
    async fn ensure_deleted_swallows_already_deleted() {
        let store = FileSystemObjectStore::in_temp_dir().unwrap();
        store
            .ensure_deleted("rechnungen/missing.pdf")
            .await
            .unwrap();
    }
}
