use armstrong::configuration::get_configuration;
use armstrong::startup::run;
use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //init calls set_logger, so all that is needed
    //printing all logs at info-level or above by default
    // if the RUST_LOG environment variable hasn't been set
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    //Panic! if we can't get/read configuration
    let configuration = get_configuration().expect("Failed to get/read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    //NB: address now coming from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
