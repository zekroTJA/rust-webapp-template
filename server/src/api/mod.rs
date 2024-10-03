mod error;
mod guards;
mod responders;
mod routes;

use crate::{config::Config, jwt};
use anyhow::Result;
use database::Database;
use error::Error;
use openid::DiscoveredClient;
use rocket::{catch, catchers, http::Status, Request};
use routes::{auth, spa};
use std::sync::Arc;

pub async fn run(cfg: Config, database: Arc<Database>) -> Result<()> {
    let oidc_client = DiscoveredClient::discover(
        cfg.oidc.id,
        Some(cfg.oidc.secret),
        Some(cfg.oidc.redirect),
        cfg.oidc.issuer,
    )
    .await?;

    let jwt_handler = jwt::Handler::new();

    rocket::build()
        .manage(oidc_client)
        .manage(jwt_handler)
        .manage(database)
        .mount("/api/auth", auth::routes())
        .mount("/", spa::routes())
        .register("/api", catchers![default_catcher])
        .launch()
        .await?;

    Ok(())
}

#[catch(default)]
fn default_catcher(status: Status, _: &Request) -> Error {
    Error::new::<&'static str>(status, status.reason_lossy(), None)
}
