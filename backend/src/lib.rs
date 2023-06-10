#[macro_use] extern crate rocket_sync_db_pools;

pub mod schema;
pub mod reviews;
pub mod listings;
pub mod posts;
pub mod videos;

use std::env;
use dotenvy::dotenv;
use rocket::serde::{Deserialize, Serialize};
use listings::{NewListing, NewListingImage};
use diesel::{prelude::*, upsert::excluded, query_dsl::methods::FilterDsl};

#[database("db")]
pub struct DbConn(PgConnection);

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}

#[derive(Debug)]
pub struct FileWatcherError {
    pub details: String,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn upsert_listings(listings: Vec<NewListing>) -> QueryResult<usize> {
    use schema::listings::*;
    let connection = &mut establish_connection();

    diesel::insert_into(table)
        .values(&listings)
        .on_conflict(id)
        .do_update()
        .set((
            price.eq(excluded(price)),
            market_st.eq(excluded(market_st)),
            updated_at.eq(excluded(updated_at))
        ))
        .filter(updated_at.ne(excluded(updated_at)))
        .execute(connection)
}

pub fn upsert_listing_images(listing_images: Vec<NewListingImage>) -> QueryResult<usize> {
    use schema::listing_images::*;
    let connection = &mut establish_connection();

    diesel::insert_into(table)
        .values(&listing_images)
        .on_conflict(id)
        .do_update()
        .set((
            url.eq(excluded(url)),
            priority.eq(excluded(priority)),
            tag.eq(excluded(tag)),
        ))
        .filter(url.ne(excluded(url)))
        .execute(connection)
}