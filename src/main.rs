pub mod server;

#[tokio::main]
async fn main() {
    server::start_server().await.expect("Failed to start the analytics server")
}
