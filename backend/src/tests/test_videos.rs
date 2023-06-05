use backend_api::videos::{Video, NewVideo};
use backend_api::{DbConn};

use rand::{Rng, thread_rng, distributions::Alphanumeric};

use crate::rocket;
use crate::make_rocket;
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};

static DB_LOCK: parking_lot::Mutex<()> = parking_lot::const_mutex(());

fn make_new_video(title: Option<String>) -> NewVideo {
    let t: String = title
        .unwrap_or("Agent Hong is truly the best of the best brokers.".to_string())
        .into();

    NewVideo {
        video_url: "https://link_to_some_image.com".to_string(),
        title: t.to_string(),
    }
}

macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => ({
        let _lock = DB_LOCK.lock();

        rocket::async_test(async move {
            let $client = Client::tracked(make_rocket()).await.expect("Rocket client");
            let db = DbConn::get_one($client.rocket()).await;
            let $conn = db.expect("failed to get database connection for testing");
            Video::delete_all(&$conn).await.expect("failed to delete all videos for testing");
            
            $block
        })
    })
}

#[test]
fn test_insertion_deletion() {
    run_test!(|client, conn| {
        // Issue a request to insert a new video
        let source = make_new_video(None);
        let mut res = client.post("/videos")
            .json(&source)
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::Created);

        // Ensure we have the inserted video in the db
        let v_id = res.into_json::<Video>().await.unwrap().id;
        let new_video = Video::get_with_id(v_id, &conn).await.unwrap();
        
        // Ensure the new video is what we expect
        assert_eq!(new_video.id, v_id);
        assert_eq!(new_video.title, source.title);
        
        // Issue a request to delete the video
        res = client.delete(format!("/videos/{}", v_id)).dispatch().await;
        assert_eq!(res.status(), Status::NoContent);

        // Ensure the video is deleted
        let e = Video::get_with_id(v_id, &conn).await;
        assert!(e.is_err());
    })
}

#[test]
fn test_many_insertions() {
    const ITER: usize = 100;

    run_test!(|client, conn| {
        // Get the initial number of videos 
        let init_num = Video::get_all(Some(100), None, &conn).await.unwrap().len();
        let mut titles = Vec::new();

        for i in 0..ITER {
            // Issue a request to insert a new video with a random    
            let title: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect();

            let source = make_new_video(Some(title.clone()));

            client.post("/videos")
                .json(&source)
                .dispatch()
                .await;

            // Record the title we chose for this iteration
            titles.insert(0, title);

            // Ensure the video was inserted properly and other videos remain
            let videos = Video::get_all(Some(100), None, &conn).await.unwrap();
            println!("{} == {}", videos.len(), init_num+i+1);
            assert_eq!(videos.len(), init_num+i+1);

            for j in 0..i {
                assert_eq!(titles[j], videos[j].title);
            }
        }
    })
}

#[test] 
fn test_bad_insertion() {
    run_test!(|client, conn| {
        // Submit no body. We should get a 400
        let res = client.post("/videos")
            .header(ContentType::JSON) 
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::BadRequest);

        // Submit an empty JSON body. We should get a 422
        let res = client.post("/videos")
            .header(ContentType::JSON)
            .body("{}")
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::UnprocessableEntity);
        
        // Submit a partially complete json body. We should still get 422
        let res = client.post("/videos")
            .header(ContentType::JSON)
            .body("{ \"title\": \"A\" }")
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::UnprocessableEntity);

        // Submit a full json body, we should get a 201
        let res = client.post("/videos")
            .header(ContentType::JSON)
            .body("{ \"video_url\": \"A\", \"title\": \"B\" }")
            .dispatch()
            .await;
        
        assert_eq!(res.status(), Status::Created);

    })
}
