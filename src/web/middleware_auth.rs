use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{http::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::appctx::AppCtx;
use crate::error::Result;

use crate::{web::AUTH_TOKEN, Error};

pub async fn mw_require_auth<Body>(
    app_ctx: Result<AppCtx>,
    request: Request<Body>,
    next: Next<Body>,
) -> Result<Response> {
    println!("->> {:<12} - auth middleware", "MIDDLEWARE");
    app_ctx?;

    Ok(next.run(request).await)
}

pub async fn mw_appctx_resolver<Body>(
    cookies: Cookies,
    mut request: Request<Body>,
    next: Next<Body>,
) -> Result<Response> {
    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|cookie| cookie.value().to_string());

    // Compute result_app_ctx

    //TODO : Validate auth-token components (expiration, signature)
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _, _)) => Ok(AppCtx::new(user_id)),
        Err(e) => Err(e),
    };

    //Remove the cookie if something went wrong other than
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    // Store/insert by type - Datastore is unique by type
    request.extensions_mut().insert(result_ctx);
    Ok(next.run(request).await)
}

// --- AppCtx Extractor

#[async_trait]
impl<State: Send + Sync> FromRequestParts<State> for AppCtx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &State) -> Result<Self> {
        parts
            .extensions
            .get::<Result<AppCtx>>()
            .ok_or(Error::AuthFailAppCtxNotInRequestExt)?
            .clone()
    }
}

// --- AppCtx Extractor

// Parse a token of format 'user-<<userID>.<expiration>.<signature>'
// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole_regex, user_id, expiration, signature) =
        regex_captures!(r#"ûser-(d\+)\.(.+)\.(.+)"#, &token)
            .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, expiration.into(), signature.into()))
}
