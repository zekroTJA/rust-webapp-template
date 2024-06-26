mod api;
mod config;
mod jwt;

use anyhow::Result;
use env_logger::Env;

#[rocket::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cfg = config::Config::load()?;

    api::run(cfg).await
}
