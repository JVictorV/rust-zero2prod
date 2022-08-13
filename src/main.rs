use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::Settings, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::new().expect("Failed to read configuration");

    let db_pool = PgPool::connect(&config.db.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", config.port);

    let listener = TcpListener::bind(address)?;

    run(listener, db_pool)?.await
}
