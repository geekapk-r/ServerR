#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate rocket;

#[macro_use]
extern crate lazy_static;

extern crate tungstenite;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod api;
pub mod models;
pub mod schema;
pub mod helpers;

lazy_static!(
    static ref DOGETOK: String = env::var("DOGETOK").unwrap_or(":doge:".to_string());
    static ref WEBHOOKS: String = env::var("WEBHOOKS").unwrap_or("".to_string());
    static ref VERBOSE: bool = {
        if let Ok(_) = env::var("VERBOSE") {
            true
        } else {
            false
        }
    };
);

fn main() {
    println!("Hello, geekapk!, Found environment(rocket environment see rocket output):");
    eprintln!("DOGETOK(Admin token): {}", *DOGETOK);
    eprintln!("WEBHOOKS(WebHooks config): {}", *WEBHOOKS);
    if *VERBOSE {
        eprintln!("Debug mode is on, do not enable this in production mode(VERBOSE)");
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    eprintln!("DATABASE_URL(Database): {}", database_url);
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
