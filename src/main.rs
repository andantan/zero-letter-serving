use env_logger::Env;
use zero::configuration::get_configuration;
use zero::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(
        Env::default().default_filter_or("debug")
    ).init();

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = sqlx::PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}