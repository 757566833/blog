use std::env;



pub fn  get_blog_origin ()-> Option<String> {
    let blog_origin = std::env::var("BLOG_ORIGIN");
    if let Ok(blog_origin) = blog_origin {
        return Some(blog_origin)
    } else {
        return None
    }
}

pub fn  get_auth_origin ()-> Option<String> {
    let auth_origin = std::env::var("AUTH_ORIGIN");
    if let Ok(auth_origin) = auth_origin {
        return Some(auth_origin)
    } else {
        return None
    }
}

pub struct Environment;

impl Environment {
    pub fn get_opentelemetry_server_url() -> String {
        env::var("OPENTELEMETRY_SERVER_URL").unwrap_or("".to_string())
    }
    pub fn get_cookie_key() -> String {
        env::var("COOKIE_KEY").unwrap_or("AI_NPC".to_string())
    }
}
