mod error;
mod responders;
mod routes;

use crate::config::Config;
use anyhow::Result;
use openid::DiscoveredClient;
use routes::auth;

pub async fn run(cfg: Config) -> Result<()> {
    let oidc_client = DiscoveredClient::discover(
        cfg.oidc.id,
        Some(cfg.oidc.secret),
        Some(cfg.oidc.redirect),
        cfg.oidc.issuer,
    )
    .await?;

    rocket::build()
        .manage(oidc_client)
        .mount("/api/auth", auth::routes())
        .launch()
        .await?;

    Ok(())
}
