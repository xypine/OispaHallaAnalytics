pub mod server;

#[tokio::main]
async fn main() {
    match dotenv::dotenv() {
        Ok(_) => {
            println!("dotenv loaded");
        }
        _ => {}
    }
    println!("Starting analytics server...");
    let https_bind = std::env::var("HTTPS_BIND");
    let use_https = https_bind.is_ok();
    if use_https {
        println!("Using https as HTTPS_BIND is set...");
    }
    server::start_server(use_https)
        .await
        .expect("Failed to start the analytics server");
}
