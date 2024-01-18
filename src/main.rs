use sqlx::postgres::PgPoolOptions;

use std::net::TcpListener;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't get our configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address).expect("Failed to bind to socket address.");
    run(listener, connection_pool)?.await
}
