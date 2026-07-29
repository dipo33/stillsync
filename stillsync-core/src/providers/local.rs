use super::SyncProvider;
use async_trait::async_trait;
use std::{
    io::{self},
    path::{Path, PathBuf},
};
use tokio::io::AsyncReadExt;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

pub struct LocalSync {
    base_path: PathBuf,
}

impl LocalSync {
    pub fn new(base_path: PathBuf) -> Self {
        LocalSync { base_path }
    }
}

#[async_trait]
impl SyncProvider for LocalSync {
    async fn upload(&self, file_path: &Path, data: Vec<u8>) -> Result<(), io::Error> {
        let mut destination_file = File::create(self.base_path.join(file_path)).await?;
        destination_file.write_all(&data).await?;

        Ok(())
    }

    async fn download(&self, file_path: &Path) -> Result<Vec<u8>, io::Error> {
        let mut source_file = File::open(self.base_path.join(file_path)).await?; // PROLLY CANT BE DONE IF ITS FILE
        let mut buffer = Vec::new();
        source_file.read_to_end(&mut buffer).await?;

        Ok(buffer)
    }

    async fn delete(&self, file_path: &Path) -> Result<(), io::Error> {
        fs::remove_file(self.base_path.join(file_path)).await?;

        Ok(())
    }

    fn provider_name(&self) -> String {
        "Local".to_string()
    }
}
