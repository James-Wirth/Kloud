use std::sync::Arc;
use std::env;
use tokio::sync::Mutex;
use warp::Filter;

mod api;
mod encryption;
mod storage;
mod errors;

use storage::FileStorage;

#[tokio::main]
async fn main() {
    // Load configuration
    let port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .expect("APP_PORT must be a valid u16 integer");

    let address = [127, 0, 0, 1];

    // Create shared storage for files
    let storage = Arc::new(Mutex::new(FileStorage::new()));

    // Load API routes
    let routes = api::file::routes(storage.clone())
        .with(warp::log("kloud::api"));

    // Start the server with graceful shutdown
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    let server = warp::serve(routes).run((address, port));
    let graceful = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for shutdown signal");
        println!("Shutdown signal received. Stopping server...");
        let _ = shutdown_tx.send(());
    };

    println!(
        "Starting server on http://{}:{}...",
        address.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("."),
        port
    );

    tokio::select! {
        _ = server => {},
        _ = graceful => {
            shutdown_rx.await.ok();
            println!("Server shut down gracefully.");
        }
    }
}
