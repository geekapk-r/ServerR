use rocket_contrib::{Json, Value};

use std::borrow::Borrow;

use rocket::http::ContentType;
use rocket::response::content::Content;

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
fn check_passhash(
    uid: i16,
    passhash: String,
    db: State<Pool>,
) -> Result<&'static str, Accepted<&'static str>> {
    Ok("NOTIMP")
}

#[post("/validate-metapass/<uid>", data = "<metapass>")]
fn check_metapass(
    uid: i16,
    metapass: String,
    db: State<Pool>,
) -> Result<&'static str, Accepted<&'static str>> {
    Ok("NOTIMP")
}

#[put("/pass/<uid>/<new_passhash>", data = "<metapass>")]
fn update_passhash(
    uid: i16,
    metapass: String,
    new_passhash: String,
    db: State<Pool>,
) -> Json<Value> {
    Json(json!({
        "status": 418,
        "message": "I'm not implemented"
    }))
}

#[get("/webhooks")]
fn webhooks_conf() -> String {
    (*(::WEBHOOKS)).to_owned()
}

pub mod doge {
    use db::Pool;
    use rocket::http::Cookies;
    use rocket::State;
    use rocket_contrib::{Json, Value};

    #[derive(Deserialize)]
    struct UserPost {
        simple_name: String,
        avatar_url: Option<String>,
        user_name: Option<String>,
        alias: Option<String>,
        github: String,
        bio: Option<String>,
        dev: Option<String>,
    }

    #[post("/useradd", data = "<user>")]
    fn useradd(user: Json<UserPost>, db: State<Pool>, cookies: Cookies) -> Json<Value> {
        Json(json!({
            "status": 418,
            "message": "not implemented"
        }))
    }

    #[delete("/user/<uid>")]
    fn delete_user(uid: i16, db: State<Pool>, cookies: Cookies) -> Json<Value> {
        Json(json!({
            "status": 418,
            "message": "I'm not implemented"
        }))
    }

    #[put("/user/<uid>")]
    fn endisable_user(uid: i16, cookies: Cookies, db: State<Pool>) -> Json<Value> {
        Json(json!({
            "status": 418,
            "message": "I'm not implemented"
        }))
    }

    #[put("/user/<uid>/metapass", data = "<new_metapass>")]
    fn update_metapass(
        uid: i16,
        new_metapass: String,
        cookies: Cookies,
        db: State<Pool>,
    ) -> Json<Value> {
        Json(json!({
            "status": 418,
            "message": "I'm not implemented"
        }))
    }

    #[post("/category/<parent>", data = "<name>")]
    fn create_category(
        parent: i16,
        name: String,
        cookies: Cookies,
        db: State<Pool>,
    ) -> Json<Value> {
        Json(json!({
            "status": 418,
            "message": "I'm not implemented"
        }))
    }

    #[put("/category/<tid>", data = "<new_name>")]
    fn update_category_name(
        tid: i16,
        new_name: String,
        cookies: Cookies,
        db: State<Pool>,
    ) -> Json<Value> {
        Json(json!({
            "status": 418,
            "message": "I'm not implemented"
        }))
    }

    #[put("/category/<tid>/parent", data = "<new_parent>")]
    fn update_category_parent(
        tid: i16,
        new_parent: String,
        cookies: Cookies,
        db: State<Pool>,
    ) -> Json<Value> {
        Json(json!({
            "status": 418,
            "message": "I'm not implemented"
        }))
    }

    #[delete("/category/<tid>")]
    fn delete_category(tid: i16, cookies: Cookies, db: State<Pool>) -> Json<Value> {
        Json(json!({
            "status": 418,
            "message": "I'm not implemented"
        }))
    }

    #[delete("/comment/<cid>")]
    fn delete_comment(cid: i32, cookies: Cookies, db: State<Pool>) -> Json<Value> {
        Json(json!({
            "status": 418,
            "message": "I'm not implemented"
        }))
    }

    #[delete("/app/<aid>")]
    fn delete_app(aid: i16, cookies: Cookies, db: State<Pool>) -> Json<Value> {
        Json(json!({
            "status": 418,
            "message": "I'm not implemented"
        }))
    }
}
