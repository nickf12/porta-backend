use crate::auth::AUTH_TOKEN;
use crate::{AppError, Result};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

pub async fn mw_require_auth<B>(
    //ctx: Result<Ctx>,
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth -", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    auth_token.ok_or(AppError::AuthFailNoAuthTokenCookie)?;
    Ok(next.run(req).await)
}
