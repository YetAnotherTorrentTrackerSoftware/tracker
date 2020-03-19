use crate::config::YaatsConfig;
use actix_web::{App, HttpServer};

mod config;
mod models;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config =
        YaatsConfig::new().map_err(|err| format!("Error reading configuration: {}", err))?;

    HttpServer::new(move || App::new())
        .bind(config.listen_address)?
        .run()
        .await?;

    Ok(())
}
