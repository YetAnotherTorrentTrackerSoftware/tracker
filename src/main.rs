use crate::config::YaatsConfig;
use actix_web::{middleware, App, HttpServer};

mod bencode;
mod config;
mod handlers;
mod models;

#[derive(Clone)]
pub struct AppState {
    config: YaatsConfig,
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config =
        YaatsConfig::new().map_err(|err| format!("Error reading configuration: {}", err))?;

    let app_state = AppState {
        config: config.clone(),
    };

    std::env::set_var(
        "RUST_LOG",
        config.log_level.unwrap_or_else(|| "info".to_owned()),
    );
    pretty_env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .wrap(middleware::Logger::default())
            .configure(handlers::configure)
    })
    // Start the server with a certain amount of worker threads if the corresponding option is set.
    // By default, one worker  thread per logical core is started.
    .workers(config.worker_threads.unwrap_or_else(num_cpus::get))
    .bind(config.listen_address)?
    .run()
    .await?;

    Ok(())
}
