use crate::route::NoteAppExtension;

pub async fn with_extension(
    mut req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, axum::http::StatusCode> {
    let headers= req.headers();
    let default_uid = axum::http::HeaderValue::from_static("");
    let authorization = headers
        .get(axum::http::header::AUTHORIZATION)
        .unwrap_or(&default_uid);
    let uid = authorization.to_str().unwrap_or("").to_string();

   if let Some(ext) = req.extensions_mut().get_mut::<NoteAppExtension>() {
        // 修改 uid
        ext.uid = uid;
    }
    let response = next.run(req).await;
    Ok(response)
}

