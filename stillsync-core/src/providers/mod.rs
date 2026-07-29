pub mod local;

use async_trait::async_trait;
use std::{io, path::Path};

#[async_trait]
pub trait SyncProvider: Send + Sync {
    async fn upload(&self, file_path: &Path, data: Vec<u8>) -> Result<(), io::Error>;
    async fn download(&self, file_path: &Path) -> Result<Vec<u8>, io::Error>;
    async fn delete(&self, file_path: &Path) -> Result<(), io::Error>;
    fn provider_name(&self) -> String;
}
