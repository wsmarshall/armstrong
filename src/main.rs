use armstrong::configuration::get_configuration;
use armstrong::startup::run;
use sqlx::postgres::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

/// Compose multiple layers into a 'tracing' subscriber
///
/// # Implementation Notes
///
/// using 'impl Subscriber' ar return type, this avoids needing to
/// specify actual type of the returned subscriber, which ...gets messy
/// Need to spicify the returned subscriber is Send and Sync, which
/// makes it possible to pass it to 'init subscriber' later
pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

///Register a subscriber as global default to process span data
///
/// NB this should only get called once
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //redirects all 'log' events to our subscriber

    LogTracer::init().expect("Failed to set logger");
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
