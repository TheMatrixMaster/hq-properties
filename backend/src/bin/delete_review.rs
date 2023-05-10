use self::reviews::Review;
use diesel::prelude::*;
use backend_api::*;
use std::env::args;

fn main() {
    use self::schema::reviews::dsl::{reviews};

    let id = args()
        .nth(1)
        .expect("delete_review requires a review id")
        .parse::<i32>()
        .expect("Invalid ID");
    
    let connection = &mut establish_connection();
    let deleted = diesel::delete(reviews.find(id))
        .get_result::<Review>(connection)
        .unwrap_or_else(|_| panic!("Failed to delete review with id {}", id));

    println!("Deleted review with id {}", deleted.id);
}


