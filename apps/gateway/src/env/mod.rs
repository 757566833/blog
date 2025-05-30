use std::env;

pub fn get_note_origin() -> Option<String> {
    let note_origin = std::env::var("NOTE_ORIGIN");
    if let Ok(note_origin) = note_origin {
        return Some(note_origin);
    } else {
        return None;
    }
}

pub fn get_auth_origin() -> Option<String> {
    let auth_origin = std::env::var("AUTH_ORIGIN");
    if let Ok(auth_origin) = auth_origin {
        return Some(auth_origin);
    } else {
        return None;
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
