use tempfile::NamedTempFile;

use crate::providers::local::LocalSync;

use std::{
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::providers::SyncProvider;

pub struct SyncService {
    //    config: Config,
    //    ipc_server: IpcServer,
    source_provider: Arc<dyn SyncProvider>,
    destination_provider: Arc<dyn SyncProvider>,
}

impl SyncService {
    pub fn new(/*config: Config*/) -> Self {
        // TODO: Remove this
        //let tmp_source = NamedTempFile::new().unwrap();
        //let tmp_destination = NamedTempFile::new().unwrap();

        let mut f = File::create("/tmp/source").unwrap();
        let _ = f.write_all("Some random test file".as_bytes());

        // Based on config, initialize the appropriate providers
        let source_provider: Arc<dyn SyncProvider> = Arc::new(LocalSync::new(PathBuf::from("/tmp/source")));
        let destination_provider: Arc<dyn SyncProvider> = Arc::new(LocalSync::new(PathBuf::from("/tmp/dest")));

        //        // Define a socket path or pipe name for the IPC server
        //        let ipc_server = if cfg!(unix) {
        //            IpcServer::new("/tmp/sync_service.sock".to_string())
        //        } else {
        //            IpcServer::new(r"\\.\pipe\sync_service".to_string())
        //        };

        SyncService {
            source_provider,
            destination_provider,
            //            ipc_server,
            //            config,
        }
    }

    pub async fn start(&mut self) -> Result<(), io::Error> {
        //        // Start the IPC server and sync operations
        //        tokio::spawn(async move {
        //            self.ipc_server.run().await;
        //        });

        // Start syncing
        self.sync().await?;

        Ok(())
    }

    async fn sync(&self) -> Result<(), io::Error> {
        // Coordinate sync between the source and destination providers
        let data = self.source_provider.download(Path::new("")).await?;
        self.destination_provider.upload(Path::new(""), data).await?;

        Ok(())
    }
}
