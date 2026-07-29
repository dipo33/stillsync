use sync_service::SyncService;

mod providers;
mod sync_service;

#[tokio::main]
async fn main() {
    println!("Starting");
    let mut service = SyncService::new();
    let _ = service.start().await;
    println!("Exiting")
}
