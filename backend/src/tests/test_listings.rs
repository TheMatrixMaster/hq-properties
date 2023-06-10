use backend_api::listings::{Listing, NewListing, ListingImage, NewListingImage, MarketStatus};
use backend_api::{DbConn};

use chrono::{NaiveDateTime};
use rand::{Rng, thread_rng, distributions::Alphanumeric};

use crate::rocket;
use crate::make_rocket;
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};

static DB_LOCK: parking_lot::Mutex<()> = parking_lot::const_mutex(());

fn make_new_listing_img(id: i32, listing_id: i32) -> NewListingImage {
    NewListingImage {
        id,
        listing_id: listing_id,
        url: "https://ts1.cn.mm.bing.net/th/id/R-C.652b794060c61d86c1f9c94efbc7b6ee?rik=8rmgkkPC6irH1w&pid=ImgRaw&r=0".to_string(),
        priority: 1,
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
            Listing::delete_all(&$conn).await.expect("failed to delete all listings for testing");
            
            $block
        })
    })
}

// TODO write tests when I've completed the file watcher