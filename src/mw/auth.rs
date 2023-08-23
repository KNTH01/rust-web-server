use crate::{ctx::Ctx, model::ModelController, web, Error, Result};
use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies
        .get(web::AUTH_TOKEN)
        .map(|cookie| cookie.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailTokenNotValid)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO: token component validation (jwt?)
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        // remove the cookie if something went wrong other than no auth token cookie
        cookies.remove(Cookie::named(web::AUTH_TOKEN));
    }

    // store the result_ctx in the request extension
    debug!(
        "insert Ctx: {:?} into request",
        match result_ctx.as_ref() {
            Ok(ctx) => Some(ctx),
            Err(_) => None,
        }
    );
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!("middleware mw_require_auth called");

    ctx?;

    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenNotValid)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenNotValid)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}

// Ctx Extrator
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self> {
        req.extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotFoundInReqExt)?
            .clone()
    }
}
