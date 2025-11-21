use armstrong::configuration::get_configuration;
use armstrong::startup::run;
use armstrong::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("armstrong".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    //Panic! if we can't get/read configuration
    let configuration = get_configuration().expect("Failed to get/read configuration.");
    let connection =
        PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");
    //NB: address now coming from our settings
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
