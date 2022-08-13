use std::net::TcpListener;

use config::Config;
use zero2prod::{configuration::Settings, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::new().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", config.port);

    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
