use rocket_contrib::{Json, Value};

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": 404,
        "message": "Resource was not found"
    }))
}

#[error(413)]
fn too_large() -> Json<Value> {
    Json(json!({
        "status": 413,
        "message": "Request content too large"
    }))
}

#[error(401)]
fn unauthorized() -> Json<Value> {
    Json(json!({
        "status": 401,
        "message": "Unauthorized, check geekapk_uid/geekapk_passhash"
    }))
}

#[error(400)]
fn bad_request() -> Json<Value> {
    Json(json!({
       "status": 400,
       "message": "I can't recognize your request"
   }))
}
