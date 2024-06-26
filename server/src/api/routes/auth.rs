use crate::{
    api::{
        error::Error,
        responders::{Cookies, Either},
    },
    jwt,
};
use openid::{Client, Token};
use rocket::{
    get,
    http::{Cookie, Status},
    response::Redirect,
    routes,
    time::{Duration, OffsetDateTime},
    Route, State,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    redirect: Option<String>,
}

#[get("/login?<redirect>")]
async fn login(
    client: &State<Client>,
    jwt_handler: &State<jwt::Handler>,
    redirect: Option<String>,
) -> Result<Redirect, Error> {
    let claims = Claims {
        exp: jwt::expire_in(Duration::minutes(5)),
        redirect,
    };
    let token = jwt_handler.encode(claims)?;
    let uri = client.auth_uri(Some("openid profile"), Some(token.as_ref()));
    Ok(Redirect::temporary(uri.to_string()))
}

#[get("/callback?<code>&<state>")]
async fn callback<'r>(
    client: &State<Client>,
    jwt_handler: &State<jwt::Handler>,
    code: String,
    state: String,
) -> Result<Cookies<Either<Redirect, Status>>, Error> {
    let claims: Claims = jwt_handler.decode(&state, &["exp"])?;

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

    let inner = match claims.redirect {
        Some(redirect) => Either::A(Redirect::temporary(redirect)),
        None => Either::B(Status::NoContent),
    };

    Ok(Cookies {
        inner,
        cookies: vec![id_cookie],
    })
}

pub fn routes() -> Vec<Route> {
    routes![login, callback]
}