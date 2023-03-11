use std::net::TcpListener;

use newsletter::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", config.port);

    let listener = TcpListener::bind(address).expect("Failed to bind to port 8000");
    run(listener)?.await
}
