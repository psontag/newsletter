use std::net::TcpListener;

use newsletter::{configuration::get_configuration, startup::run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration");
    let pool = PgPool::connect(&config.database.conn_str())
        .await
        .expect("Failed to connect to DB");

    let address = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(address).expect("Failed to bind to port 8000");

    run(listener, pool)?.await
}
