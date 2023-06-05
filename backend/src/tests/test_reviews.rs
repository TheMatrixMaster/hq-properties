use backend_api::reviews::{Review, NewReview};
use backend_api::{DbConn};

use rand::{Rng, thread_rng, distributions::Alphanumeric};

use crate::rocket;
use crate::make_rocket;
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};

static DB_LOCK: parking_lot::Mutex<()> = parking_lot::const_mutex(());

fn make_new_review(body: Option<String>) -> NewReview {
    let b = body
        .unwrap_or("Agent Hong is truly the best of the best brokers.".to_string())
        .into();

    NewReview {
        first_name: "Client".to_string(),
        last_name: "A".to_string(),
        body: b,
        anonymous: false
    }
}

macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => ({
        let _lock = DB_LOCK.lock();

        rocket::async_test(async move {
            let $client = Client::tracked(make_rocket()).await.expect("Rocket client");
            let db = DbConn::get_one($client.rocket()).await;
            let $conn = db.expect("failed to get database connection for testing");
            Review::delete_all(&$conn).await.expect("failed to delete all reviews for testing");
            
            $block
        })
    })
}

#[test]
fn test_insertion_deletion() {
    run_test!(|client, conn| {
        // Issue a request to insert a new review
        let source = make_new_review(None);
        let mut res = client.post("/reviews")
            .json(&source)
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::Created);

        // Ensure we have the inserted review in the db
        let r_id = res.into_json::<Review>().await.unwrap().id;
        let new_review = Review::get_with_id(r_id, &conn).await.unwrap();
        
        // Ensure the new review is what we expect
        assert_eq!(new_review.id, r_id);
        assert_eq!(new_review.body, source.body);
        assert_eq!(new_review.anonymous, source.anonymous);
        
        // Issue a request to delete the review
        res = client.delete(format!("/reviews/{}", r_id)).dispatch().await;
        assert_eq!(res.status(), Status::NoContent);

        // Ensure the review is deleted
        let e = Review::get_with_id(r_id, &conn).await;
        assert!(e.is_err());
    })
}

#[test]
fn test_publish() {
    run_test!(|client, conn| {
        // Issue a request to insert a new review; ensure it is unpublished
        let source = make_new_review(None);
        let res = client.post("/reviews")
            .json(&source)
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::Created);
        
        // Make sure the review is unpublished
        let r_id = res.into_json::<Review>().await.unwrap().id;
        assert_eq!(Review::get_with_id(r_id, &conn).await.unwrap().published, false);
        
        // Issue a request to publish the review
        client.patch(format!("/reviews/{}", r_id)).dispatch().await;
        assert_eq!(Review::get_with_id(r_id, &conn).await.unwrap().published, true);
    })
}

#[test]
fn test_many_insertions() {
    const ITER: usize = 100;

    run_test!(|client, conn| {
        // Get the initial number of reviews 
        let init_num = Review::get_all(Some(100), None, Some(false), &conn).await.unwrap().len();
        let mut bodies = Vec::new();

        for i in 0..ITER {
            // Issue a request to insert a new review with a random    
            let body: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect();

            let source = make_new_review(Some(body.clone()));

            client.post("/reviews")
                .json(&source)
                .dispatch()
                .await;

            // Record the body we chose for this iteration
            bodies.insert(0, body);

            // Ensure the review was inserted properly and other reviews remain
            let reviews = Review::get_all(Some(100), None, Some(false), &conn).await.unwrap();
            println!("{} == {}", reviews.len(), init_num+i+1);
            assert_eq!(reviews.len(), init_num+i+1);

            for j in 0..i {
                assert_eq!(bodies[j], reviews[j].body);
            }
        }
    })
}

#[test] 
fn test_bad_insertion() {
    run_test!(|client, conn| {
        // Submit no body. We should get a 400
        let res = client.post("/reviews")
            .header(ContentType::JSON) 
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::BadRequest);

        // Submit an empty JSON body. We should get a 422
        let res = client.post("/reviews")
            .header(ContentType::JSON)
            .body("{}")
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::UnprocessableEntity);
        
        // Submit a partially complete json body. We should still get 422
        let res = client.post("/reviews")
            .header(ContentType::JSON)
            .body("{ \"first_name\": \"A\", \"last_name\": \"B\" }")
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::UnprocessableEntity);

        // Submit a full json body, we should get a 201
        let res = client.post("/reviews")
            .header(ContentType::JSON)
            .body("{ \"first_name\": \"A\", \"last_name\": \"B\", \"body\": \"testing\", \"anonymous\": false }")
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::Created);

    })
}
