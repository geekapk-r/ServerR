#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate r2d2;
extern crate r2d2_diesel;

extern crate rocket;

#[macro_use]
extern crate lazy_static;
extern crate tungstenite;

extern crate yansi;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use std::sync::Arc;

pub mod api;
pub mod db;
pub mod helpers;
pub mod models;
pub mod schema;

use db::Pool;

use yansi::Paint;

use api::*;
use helpers::*;

lazy_static! {
    static ref DOGETOK: String = env::var("DOGETOK").unwrap_or(":doge:".to_string());
    static ref WEBHOOKS: String = env::var("WEBHOOKS").unwrap_or("".to_string());
    static ref VERBOSE: bool = env::var("VERBOSE").is_ok();
}

lazy_static! {
    static ref WEB_HOOKS: Vec<WebHook> = WebHook::parse((*WEBHOOKS).to_owned());
    static ref WELCOME_DOC: String =
        format!("{}", String::from_utf8_lossy(include_bytes!("web.html")));
}

fn main() {
    println!(
        "{}{} Found environment {}:",
        Paint::masked("🌌  "),
        Paint::green("Hello, GeekApk!"),
        Paint::red("(rocket environment see rocket output)")
    );
    eprintln!(
        "{}{}: {}",
        Paint::masked("🐶  "),
        Paint::blue("DOGETOK"),
        Paint::white((*DOGETOK).to_owned()).bold()
    );
    if *DOGETOK == ":doge:" {
        eprintln!(
            "{}{}",
            Paint::masked("💣  "),
            Paint::red("USING DEFAULT ADMIN PASSWORD!!!").bold()
        );
        eprintln!(
            "{}{}",
            Paint::green("NOTE: "),
            Paint::white("Set DOGETOK environment variable to your admin password").bold()
        );
    }
    eprintln!(
        "{}{}: {}",
        Paint::masked("🔌  "),
        Paint::blue("WEBHOOKS"),
        Paint::white((*WEBHOOKS).to_owned()).bold()
    );
    if *VERBOSE {
        eprintln!(
            "{}{}",
            Paint::masked("📜  "),
            Paint::yellow("Debug mode is on, do not enable this in production mode")
        );
    }

    if *VERBOSE {
        println!("Parsed WebHooks: {:?}", *WEB_HOOKS);
    }

    rocket::ignite()
        .catch(errors![not_found, too_large, unauthorized, bad_request])
        .mount(
            "/",
            routes![
                welcome_api,
                version,
                ping,
                check_passhash,
                check_metapass,
                update_passhash,
                webhooks_conf
            ],
        )
        .mount(
            "/doge",
            routes![
                doge::useradd,
                doge::delete_user,
                doge::endisable_user,
                doge::update_metapass,
                doge::create_category,
                doge::update_category_name,
                doge::update_category_parent,
                doge::delete_category,
                doge::delete_comment,
                doge::delete_app
            ],
        )
        .mount(
            "user",
            routes![],
        )
        .mount(
            "category",
            routes![],
        )
        .mount(
            "app",
            routes![],
        )
        .mount(
            "comment",
            routes![],
        )
        .mount(
            "updates",
            routes![],
        )
        .mount(
            "timeline",
            routes![],
        )
        .mount(
            "notification",
            routes![],
        )
        .manage(Arc::clone(&establish_connection()))
        .launch();
}

pub fn establish_connection() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    eprintln!(
        "{}{}: {}",
        Paint::masked("📂  "),
        Paint::blue("DATABASE_URL"),
        Paint::white(database_url.to_owned()).bold()
    );
    db::init_db_pool(&database_url)
        .map_err(|e| e.to_string())
        .unwrap()
}
