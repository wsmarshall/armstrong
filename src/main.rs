use armstrong::configuration::get_configuration;
use armstrong::startup::run;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //Panic! if we can't get/read configuration
    let configuration = get_configuration().expect("Failed to get/read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    //NB: address now coming from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
