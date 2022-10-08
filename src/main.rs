use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;

use zero2prod::{
    configuration::Settings,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = Settings::new().expect("Failed to read configuration");

    let db_pool = PgPool::connect(config.db.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", config.port);

    let listener = TcpListener::bind(address)?;

    run(listener, db_pool)?.await
}
