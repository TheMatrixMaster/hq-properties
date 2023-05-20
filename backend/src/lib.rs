#[macro_use] extern crate rocket_sync_db_pools;

pub mod schema;
pub mod reviews;
pub mod listings;
pub mod posts;
pub mod videos;

use std::env;
use dotenvy::dotenv;
use diesel::{Connection, PgConnection};
use rocket::serde::{Deserialize, Serialize};

#[database("db")]
pub struct DbConn(PgConnection);

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
