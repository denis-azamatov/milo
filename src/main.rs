use milo::{
    configuration::get_configuration,    
    telemetry::{add_file_sink, create_subscriber, init_subscriber}, startup::run,
};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_name: String = "milo".into();
    let subscriber = create_subscriber(log_name.clone(), "info".into(), std::io::stdout);
    let subscriber = add_file_sink(subscriber, log_name);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
