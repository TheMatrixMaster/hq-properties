extern crate openssl;

#[macro_use]
extern crate diesel;
extern crate rocket;
extern crate rocket_cors;

use std::error::Error;
use rocket::http::Method;

use rocket::fs::{ FileServer, relative };
use rocket::{get, routes, Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use diesel::pg::Pg;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use backend_api::{ DbConn, establish_connection, reviews, listings, posts, videos, contact, buyers, sellers };

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!(); 

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

fn make_rocket() -> Rocket<Build> {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index])
        .mount("/reviews", routes![reviews::get, reviews::get_all, reviews::new, reviews::publish, reviews::destroy])
        .mount("/listings", routes![listings::get, listings::get_all])
        .mount("/posts", routes![posts::get, posts::get_all, posts::new, posts::destroy])
        .mount("/videos", routes![videos::get, videos::get_all, videos::new, videos::destroy])
        .mount("/contact", routes![contact::get, contact::get_all, contact::new, contact::destroy])
        .mount("/buyers", routes![buyers::get, buyers::get_all, buyers::new, buyers::destroy])
        .mount("/sellers", routes![sellers::get, sellers::get_all, sellers::new, sellers::destroy])
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let connection = &mut establish_connection();
    let _ = run_migrations(connection);

    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:5173",
        "https://hqproperties.ca",
        "https://www.hqproperties.ca"
    ]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Patch,
            Method::Delete
        ].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    make_rocket()
        .attach(cors)
        .launch()
        .await?;
        
    Ok(())
}

#[cfg(test)]
mod tests;

