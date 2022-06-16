use sqlx::PgPool;
use std::net::TcpListener;

use newsletter::{configuration::get_configuration, startup::run, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("newsletter".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
