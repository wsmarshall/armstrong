use armstrong::configuration::get_configuration;
use armstrong::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //printing all logs at info-level or above by default
    // if the RUST_LOG environment variable hasn't been set
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "armstrong".into(),
        //output the formatted spans to stdout
        std::io::stdout,
    );

    //'SubscriberExt' provides the 'with' method,
    //an extension trait for 'Subscriber' exposed by 'tracing_subscriber'

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    //'set_global_default' can be used by applications to specify
    //what subscriber should be used to process spans
    set_global_default(subscriber).expect("Failed to set subscriber");

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
