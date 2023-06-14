use std::convert::TryInto;

use backend_api::*;
use backend_api::listings::{Listing, ListingImage, NewListing, NewListingImage, MarketStatus};

use chrono::{NaiveDateTime};

use crate::rocket;
use crate::make_rocket;
use rocket::local::asynchronous::Client;

static DB_LOCK: parking_lot::Mutex<()> = parking_lot::const_mutex(());

fn make_new_listing_img(id: i32, listing_id: i32, url: Option<String>, priority: Option<i16>) -> NewListingImage {
    let u = 
        url.unwrap_or("https://ts1.cn.mm.bing.net/th/id/R-C.652b794060c61d86c1f9c94efbc7b6ee?rik=8rmgkkPC6irH1w&pid=ImgRaw&r=0".to_string());
    let p = priority.unwrap_or_default();

    NewListingImage {
        id,
        listing_id,
        url: u,
        priority: p,
        tag: Some("front".to_string()),
    }
}

fn make_new_listing(id: i32, city: Option<String>, market_st: Option<MarketStatus>) -> NewListing {
    let c = city.unwrap_or("Montreal, QC".to_string()).into();
    let m = market_st.unwrap_or(MarketStatus::Sale).into();

    NewListing {
        id,
        city: c,
        address: "4847 Bixby Creek Road, Carmel, CA, 93940".to_string(),
        listing_url: "https://passerelle.centris.ca/redirect.aspx?CodeDest=ROYALLEPAGE&NoMLS=14918395".to_string(),
        bedrooms: 3,
        bathrooms: 2,
        area: 600.0,
        price: 820000,
        market_st: m,
        updated_at: NaiveDateTime::default(),
    }
}

macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => ({
        let _lock = DB_LOCK.lock();

        rocket::async_test(async move {
            let $client = Client::tracked(make_rocket()).await.expect("Rocket client");
            let db = DbConn::get_one($client.rocket()).await;
            let $conn = db.expect("failed to get database connection for testing");
            
            ListingImage::delete_all(&$conn).await.expect("failed to delete all listing images for testing");
            Listing::delete_all(&$conn).await.expect("failed to delete all listings for testing");
            
            $block
        })
    })
}

#[test]
fn test_upsert_listings() {
    run_test!(|client, conn| {
        let listings_a = (1..10).map(|i| make_new_listing(i, None, None)).collect::<Vec<NewListing>>();
        let batch_a = match upsert_listings(listings_a) {
            Ok(v) => v,
            Err(e) => { println!("{e}"); panic!("Failed to insert listings into database!") }
        };
    
        assert_eq!(batch_a, 9);

        let listings_b = (1..10).map(|i| make_new_listing(i, None, None)).collect::<Vec<NewListing>>();
        let batch_b = match upsert_listings(listings_b) {
            Ok(v) => v,
            Err(e) => { println!("{e}"); panic!("Failed to insert listings into database!") }
        };

        assert_eq!(batch_b, 0);
    })
}

#[test]
fn test_upsert_listing_images() {
    run_test!(|client, conn| {
        let listings = (1..10).map(|i| make_new_listing(i, None, None)).collect::<Vec<NewListing>>();
        upsert_listings(listings).expect("Failed to insert listings into database");

        let batch_a = (1..10)
            .map(|listing_id| (1..5).map(move |id| {
                let nid = (listing_id-1)*4 + id;
                make_new_listing_img(nid, listing_id, None, Some(id.try_into().unwrap()))
            }))
            .flatten()
            .collect::<Vec<NewListingImage>>();

        let out_a = match upsert_listing_images(batch_a) {
            Ok(v) => v,
            Err(e) => { println!("{e}"); panic!("Failed to insert images into database!") }
        };

        assert_eq!(out_a, 36);  // inserted images into db

        let batch_b = (1..10)
            .map(|listing_id| (1..5).map(move |id| {
                let nid = (listing_id-1)*4 + id;
                let p = id+5;
                make_new_listing_img(nid, listing_id, None, Some(p.try_into().unwrap()))
            }))
            .flatten()
            .collect::<Vec<NewListingImage>>();

        let out_b = match upsert_listing_images(batch_b) {
                Ok(v) => v,
                Err(e) => { println!("{e}"); panic!("Failed to insert images into database!") }
            };
    
        assert_eq!(out_b, 0);   // fail due to colliding id 

        let batch_c = (1..10)
        .map(|listing_id| (1..5).map(move |id| {
            let nid = 36 + (listing_id-1)*4 + id;
            let url = "https://some_new_image_url.jpeg".to_string();
            make_new_listing_img(nid, 5, Some(url), Some(id.try_into().unwrap()))
        }))
        .flatten()
        .collect::<Vec<NewListingImage>>();

        let out_c = match upsert_listing_images(batch_c) {
            Ok(v) => v,
            Err(e) => { println!("{e}"); panic!("Failed to insert images into database!") }
        };

        assert_eq!(out_c, 0);   // fail due to non-existing listing id

        let batch_d = (1..10)
        .map(|listing_id| (1..5).map(move |id| {
            let nid = 36 + (listing_id-1)*4 + id;
            let url = "https://some_new_image_url.jpeg".to_string();
            make_new_listing_img(nid, listing_id, Some(url), Some(id.try_into().unwrap()))
        }))
        .flatten()
        .collect::<Vec<NewListingImage>>();

        let out_d = match upsert_listing_images(batch_d) {
            Ok(v) => v,
            Err(e) => { println!("{e}"); panic!("Failed to insert images into database!") }
        };

        assert_eq!(out_d, 0);   // fail due to non-unique listing_id + priority pair

        let batch_e = (1..10)
        .map(|listing_id| (1..5).map(move |id| {
            let p = id+5;
            let nid = 36 + (listing_id-1)*4 + id;
            let url = "https://some_new_image_url.jpeg".to_string();
            make_new_listing_img(nid, listing_id, Some(url), Some(p.try_into().unwrap()))
        }))
        .flatten()
        .collect::<Vec<NewListingImage>>();

        let out_e = match upsert_listing_images(batch_e) {
            Ok(v) => v,
            Err(e) => { println!("{e}"); panic!("Failed to insert images into database!") }
        };

        assert_eq!(out_e, 36);   // should successfully insert
    })
}