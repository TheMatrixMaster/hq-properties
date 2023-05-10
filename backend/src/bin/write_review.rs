use backend_api::*;
use diesel::RunQueryDsl;
use crate::schema::reviews::table;
use crate::reviews::{NewReview};
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut fname = String::new();
    let mut lname = String::new();
    let mut body = String::new();
    let anonymous = false;

    println!("What is your first name?");
    stdin().read_line(&mut fname).unwrap();
    let fname = fname.trim_end().to_string();

    println!("What is your last name?");
    stdin().read_line(&mut lname).unwrap();
    let lname = lname.trim_end().to_string();


    println!("\nOk! Let's write the review (Press {EOF} when finished)\n",);
    stdin().read_to_string(&mut body).unwrap();
    
    let review = NewReview { first_name: fname, last_name: lname, body, anonymous };

    diesel::insert_into(table)
        .values(&review)
        .execute(connection)
        .expect("Failed to insert review into db!");

    println!("\nSuccessfully saved review!");
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
