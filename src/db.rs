use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;
use std::sync::Arc;

pub type Pool = Arc<r2d2::Pool<r2d2_diesel::ConnectionManager<PgConnection>>>;

/// Initializes the database pool.
///
/// This will return a `Result` with a freshly initialized database pool inside.
///
/// # Error
///
/// In case a `Pool` can't be initialized (for whatever reason), we return a
/// `r2d2::InitializationError`.
pub fn init_db_pool(database_url: &str) -> Result<Pool, r2d2::Error> {
    let manager = r2d2_diesel::ConnectionManager::<PgConnection>::new(database_url);
    let config = r2d2::Pool::builder().max_size(15).build(manager)?;
    Ok(Arc::new(config))
}

pub struct DB(r2d2::PooledConnection<r2d2_diesel::ConnectionManager<PgConnection>>);

impl Deref for DB {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DB, ()> {
        let pool = match <State<Pool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(DB(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
