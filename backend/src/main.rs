#[macro_use] extern crate rocket;

use rocket::fs::{ FileServer, relative };
use backend_api::{ DbConn, reviews, listings, posts, videos };

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index])
        .mount("/reviews", routes![reviews::get, reviews::get_all, reviews::new, reviews::publish, reviews::destroy])
        .mount("/listings", routes![listings::get, listings::get_all])
        .mount("/posts", routes![posts::get, posts::get_all, posts::new, posts::destroy])
        .mount("/videos", routes![videos::get, videos::get_all, videos::new, videos::destroy])
}

#[cfg(test)]
mod tests;

