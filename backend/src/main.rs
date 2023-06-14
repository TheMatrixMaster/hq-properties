#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use std::error::Error;
use rocket::http::Method;
use rocket::{get, routes, Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use rocket::fs::{ FileServer, relative };
use backend_api::{ DbConn, reviews, listings, posts, videos, contact, buyers, sellers };

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:5173",
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

