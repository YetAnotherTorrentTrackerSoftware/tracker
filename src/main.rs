use crate::config::YaatsConfig;
use actix_web::{web, App, HttpServer};

mod bencode;
mod config;
mod handlers;
mod models;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config =
        YaatsConfig::new().map_err(|err| format!("Error reading configuration: {}", err))?;

    HttpServer::new(move || {
        App::new().route("/announce", web::get().to(handlers::handle_announce))
    })
        // Start the server with a certain amount of worker threads if the corresponding option is set.
        // By default, one worker  thread per logical core is started.
        .workers(config.worker_threads.unwrap_or(num_cpus::get()))
        .bind(config.listen_address)?
        .run()
        .await?;

    Ok(())
}
