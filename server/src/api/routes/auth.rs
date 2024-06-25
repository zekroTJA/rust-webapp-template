use crate::api::{error::Error, responders::Cookies};
use openid::{Client, Token};
use rocket::{
    get,
    http::{Cookie, Status},
    response::Redirect,
    routes,
    time::{Duration, OffsetDateTime},
    Route, State,
};

#[get("/login")]
async fn login(client: &State<Client>) -> Redirect {
    let uri = client.auth_uri(Some("openid profile"), None);
    Redirect::temporary(uri.to_string())
}

#[get("/callback?<code>")]
async fn callback(client: &State<Client>, code: String) -> Result<Cookies<Status>, Error> {
    let mut token: Token = client.request_token(&code).await?.into();

    let Some(id_token) = token.id_token.as_mut() else {
        return Err(Error::message(
            Status::BadRequest,
            "no ID token contained in response",
        ));
    };

    let id_token_b64 = id_token.encoded().unwrap().to_string();

    client.decode_token(id_token)?;
    client.validate_token(id_token, None, None)?;

    let id_cookie = Cookie::build(("id_token", id_token_b64))
        .expires(OffsetDateTime::now_utc() + Duration::weeks(4))
        .http_only(true)
        .build();

    Ok(Cookies {
        inner: Status::NoContent,
        cookies: vec![id_cookie],
    })
}

pub fn routes() -> Vec<Route> {
    routes![login, callback]
}
