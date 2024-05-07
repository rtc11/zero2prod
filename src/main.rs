use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::config::get_config;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config()
        .expect("Failed to read configuration");

    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(address)
        .expect("Failed to bind random port");

    run(listener, connection_pool)?.await
}

