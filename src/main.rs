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

use api::*;
use helpers::*;

lazy_static! {
    static ref DOGETOK: String = env::var("DOGETOK").unwrap_or(":doge:".to_string());
    static ref WEBHOOKS: String = env::var("WEBHOOKS").unwrap_or("".to_string());
    static ref VERBOSE: bool = env::var("VERBOSE").is_ok();
}

lazy_static! {
    static ref WEB_HOOKS: Vec<WebHook> = WebHook::parse((*WEBHOOKS).to_owned());
}

fn main() {
    println!("Hello, geekapk!, Found environment(rocket environment see rocket output):");
    eprintln!("DOGETOK(Admin token): {}", *DOGETOK);
    eprintln!("WEBHOOKS(WebHooks config): {}", *WEBHOOKS);
    if *VERBOSE {
        eprintln!("Debug mode is on, do not enable this in production mode(VERBOSE)");
    }

    if *VERBOSE {
        println!("Parsed WebHooks: {:?}", *WEB_HOOKS);
    }
    rocket::ignite()
        .catch(errors![not_found, too_large, unauthorized, bad_request])
        .manage(Arc::clone(&establish_connection()))
        .launch();
}

pub fn establish_connection() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    eprintln!("DATABASE_URL(Database): {}", database_url);
    db::init_db_pool(&database_url)
        .map_err(|e| e.to_string())
        .unwrap()
}
