use rocket_contrib::{Json, Value};

use std::borrow::Borrow;

use rocket::response::content::Content;
use rocket::http::ContentType;

use rocket::Request;
use rocket::State;

use rocket::response::status::*;

use db::Pool;

use helpers::*;

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

#[get("/")]
fn welcome_api() -> Content<String> {
    Content(ContentType::HTML, (*(::WELCOME_DOC)).to_owned())
}

#[get("/version")]
fn version() -> &'static str {
    "1:0.1.0"
}

#[get("/ping")]
fn ping() -> &'static str {
    "PING!"
}

#[post("/validate/<uid>", data = "<passhash>")]
fn check_passhash(uid: i16, passhash: String, db: State<Pool>) -> Result<&'static str, Accepted<&'static str>> {
    Ok("NOTIMP")
}

#[post("/validate-metapass/<uid>", data = "<metapass>")]
fn check_metapass(uid: i16, metapass: String, db: State<Pool>) -> Result<&'static str, Accepted<&'static str>> {
    Ok("NOTIMP")
}

#[put("/pass/<uid>", data = "<metapass>")]
fn update_passhash(uid: i16, metapass: String, db: State<Pool>) -> Json<Value> {
    Json(json!({
        "status": 418,
        "message": "I'm not implemented"
    }))
}

#[get("/webhooks")]
fn webhooks_conf() -> String {
    (*(::WEBHOOKS)).to_owned()
}
