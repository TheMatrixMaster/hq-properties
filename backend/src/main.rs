#[macro_use] extern crate rocket;

use rocket::fs::{ FileServer, relative };
use backend_api::{ DbConn, reviews };

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
}

#[cfg(test)]
mod tests;

