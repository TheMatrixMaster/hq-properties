// @generated automatically by Diesel CLI.

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
