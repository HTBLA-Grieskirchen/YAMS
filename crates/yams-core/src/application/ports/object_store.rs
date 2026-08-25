use std::pin::Pin;

use async_trait::async_trait;
use futures_lite::{Stream, StreamExt, stream};

use crate::ResultReport;
use crate::domain::{RechnungId, SeminarBuchungId, SeminarTerminId};

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
}

pub fn rechnung_object_key(id: &RechnungId) -> String {
    format!("rechnungen/{}.pdf", id.0)
}

pub fn teilnahme_object_key(termin_id: &SeminarTerminId, buchung_id: &SeminarBuchungId) -> String {
    format!(
        "teilnahmebestaetigungen/{}/{}.pdf",
        termin_id.0, buchung_id.0
    )
}

pub async fn collect_object(mut stream: ObjectStream) -> Result<Vec<u8>, ObjectStoreError> {
    let mut out = Vec::new();
    while let Some(chunk) = stream.next().await {
        out.extend(chunk?);
    }
    Ok(out)
}

pub fn once_stream(bytes: Vec<u8>) -> ObjectStream {
    Box::pin(stream::once(Ok(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn rechnung_key_uses_uuid() {
        let id = RechnungId(Uuid::nil());
        assert_eq!(
            rechnung_object_key(&id),
            "rechnungen/00000000-0000-0000-0000-000000000000.pdf"
        );
    }

    #[test]
    fn teilnahme_key_nests_termin_and_buchung() {
        let termin = SeminarTerminId(Uuid::nil());
        let buchung = SeminarBuchungId(Uuid::from_u128(1));
        assert_eq!(
            teilnahme_object_key(&termin, &buchung),
            "teilnahmebestaetigungen/00000000-0000-0000-0000-000000000000/00000000-0000-0000-0000-000000000001.pdf"
        );
    }
}
