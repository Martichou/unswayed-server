use actix_web::HttpRequest;

pub fn get_user_id(req: &HttpRequest) -> Option<&str> {
    req.headers().get("user_id")?.to_str().ok()
}
