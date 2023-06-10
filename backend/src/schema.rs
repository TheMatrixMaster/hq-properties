// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "market_status"))]
    pub struct MarketStatus;
}

diesel::table! {
    contact_messages (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        body -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    listing_images (id) {
        id -> Int4,
        listing_id -> Int4,
        url -> Text,
        priority -> Int2,
        tag -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MarketStatus;

    listings (id) {
        id -> Int4,
        city -> Varchar,
        address -> Varchar,
        listing_url -> Text,
        bedrooms -> Int2,
        bathrooms -> Int2,
        area -> Float8,
        price -> Int4,
        market_st -> MarketStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        img_url -> Text,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        body -> Text,
        anonymous -> Bool,
        published -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    videos (id) {
        id -> Int4,
        video_url -> Text,
        title -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(listing_images -> listings (listing_id));

diesel::allow_tables_to_appear_in_same_query!(
    contact_messages,
    listing_images,
    listings,
    posts,
    reviews,
    videos,
);
