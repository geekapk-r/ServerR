#[error(404)]
fn not_found() -> &'static str {
    "{\"status\":404,\"message\":\"Not Found\"}"
}

#[error(413)]
fn too_large() -> &'static str {
    "{\"status\":413,\"message\":\"Payload Too Large\"}"
}

#[error(401)]
fn unauthorized() -> &'static str {
    "{\"status\":401,\"message\":\"Unauthorized\"}"
}

#[error(400)]
fn bad_request() -> &'static str {
    "{\"status\":400,\"message\":\"Bad Request\"}"
}
