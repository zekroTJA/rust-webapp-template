use anyhow::Result;
use rocket::figment::{providers::Env, Figment};
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub oidc: Oidc,
}

#[derive(Deserialize, Debug)]
pub struct Oidc {
    pub id: String,
    pub secret: String,
    pub redirect: String,
    pub issuer: Url,
}

impl Config {
    pub fn load() -> Result<Self> {
        let cfg = Figment::new()
            .merge(Env::prefixed(env!("CONFIG_PREFIX")).split('_'))
            .extract()?;

        Ok(cfg)
    }
}
