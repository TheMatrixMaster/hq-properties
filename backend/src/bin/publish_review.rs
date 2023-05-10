use self::reviews::Review;
use diesel::prelude::*;
use backend_api::*;
use std::env::args;

fn main() {
    use self::schema::reviews::dsl::{reviews, published};

    let id = args()
        .nth(1)
        .expect("publish_review requires a review id")
        .parse::<i32>()
        .expect("Invalid ID");
    
    let connection = &mut establish_connection();
    let review = diesel::update(reviews.find(id))
        .set(published.eq(true))
        .get_result::<Review>(connection)
        .unwrap();
    println!("Published review with id {}", review.id);
}

