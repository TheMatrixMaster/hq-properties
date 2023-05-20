use backend_api::posts::{Post, NewPost};
use backend_api::{DbConn};

use rand::{Rng, thread_rng, distributions::Alphanumeric};

use crate::rocket;
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};

static DB_LOCK: parking_lot::Mutex<()> = parking_lot::const_mutex(());

fn make_new_post(body: Option<String>) -> NewPost {
    let b = body
        .unwrap_or("Agent Hong is truly the best of the best brokers.".to_string())
        .into();

    NewPost {
        img_url: "https://link_to_some_image.com".to_string(),
        title: "Some post title".to_string(),
        body: b,
    }
}

macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => ({
        let _lock = DB_LOCK.lock();

        rocket::async_test(async move {
            let $client = Client::tracked(rocket()).await.expect("Rocket client");
            let db = DbConn::get_one($client.rocket()).await;
            let $conn = db.expect("failed to get database connection for testing");
            Post::delete_all(&$conn).await.expect("failed to delete all posts for testing");
            
            $block
        })
    })
}

#[test]
fn test_insertion_deletion() {
    run_test!(|client, conn| {
        // Issue a request to insert a new post
        let source = make_new_post(None);
        let mut res = client.post("/posts")
            .json(&source)
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::Created);

        // Ensure we have the inserted post in the db
        let p_id = res.into_json::<Post>().await.unwrap().id;
        let new_post = Post::get_with_id(p_id, &conn).await.unwrap();
        
        // Ensure the new post is what we expect
        assert_eq!(new_post.id, p_id);
        assert_eq!(new_post.body, source.body);
        
        // Issue a request to delete the post
        res = client.delete(format!("/posts/{}", p_id)).dispatch().await;
        assert_eq!(res.status(), Status::NoContent);

        // Ensure the post is deleted
        let e = Post::get_with_id(p_id, &conn).await;
        assert!(e.is_err());
    })
}

#[test]
fn test_many_insertions() {
    const ITER: usize = 100;

    run_test!(|client, conn| {
        // Get the initial number of posts 
        let init_num = Post::get_all(Some(100), None, &conn).await.unwrap().len();
        let mut bodies = Vec::new();

        for i in 0..ITER {
            // Issue a request to insert a new post with a random    
            let body: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect();

            let source = make_new_post(Some(body.clone()));

            client.post("/posts")
                .json(&source)
                .dispatch()
                .await;

            // Record the body we chose for this iteration
            bodies.insert(0, body);

            // Ensure the post was inserted properly and other posts remain
            let posts = Post::get_all(Some(100), None, &conn).await.unwrap();
            println!("{} == {}", posts.len(), init_num+i+1);
            assert_eq!(posts.len(), init_num+i+1);

            for j in 0..i {
                assert_eq!(bodies[j], posts[j].body);
            }
        }
    })
}

#[test] 
fn test_bad_insertion() {
    run_test!(|client, conn| {
        // Submit no body. We should get a 400
        let res = client.post("/posts")
            .header(ContentType::JSON) 
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::BadRequest);

        // Submit an empty JSON body. We should get a 422
        let res = client.post("/posts")
            .header(ContentType::JSON)
            .body("{}")
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::UnprocessableEntity);
        
        // Submit a partially complete json body. We should still get 422
        let res = client.post("/posts")
            .header(ContentType::JSON)
            .body("{ \"title\": \"A\", \"body\": \"B\" }")
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::UnprocessableEntity);

        // Submit a full json body, we should get a 201
        let res = client.post("/posts")
            .header(ContentType::JSON)
            .body("{ \"img_url\": \"A\", \"title\": \"B\", \"body\": \"testing\" }")
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::Created);

    })
}
