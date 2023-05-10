use backend_api::reviews::Review;
use backend_api::establish_connection;
use diesel::prelude::*;

fn main() {
    use backend_api::schema::reviews::dsl::*;

    let connection = &mut establish_connection();
    let results = reviews
        .filter(published.eq(true))
        .limit(5)
        .load::<Review>(connection)
        .expect("Error loading reviews");

    println!("Displaying {} reviews", results.len());
    for review in results {
        println!("{} {}", review.first_name, review.last_name);
        println!("-----------\n");
        println!("{}", review.body);
    }
}
