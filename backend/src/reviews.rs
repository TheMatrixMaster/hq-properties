use diesel::prelude::*;
use diesel::result::QueryResult;

use chrono::NaiveDateTime;

use crate::schema::reviews;
use crate::schema::reviews::dsl::reviews as all_reviews;

use rocket::{get, post, patch, delete};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::response::status::{Created, NoContent, NotFound};

use super::{DbConn, ApiError};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = reviews)]
pub struct Review {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub body: String,
    pub anonymous: bool,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = reviews)]
pub struct NewReview {
    pub first_name: String,
    pub last_name: String,
    pub body: String,
    pub anonymous: bool,
}

impl Review {
    pub async fn get_with_id(id: i32, conn: &DbConn) -> QueryResult<Review> {
        conn.run(move |c| {
            all_reviews.find(id)
                .first::<Review>(c)
        }).await
    }

    pub async fn get_all(
        limit: Option<u8>,
        offset: Option<u32>,
        publ: Option<bool>,
        conn: &DbConn
    ) -> QueryResult<Vec<Review>>
    {
        conn.run(move |c| {
            all_reviews
                .filter(reviews::published.eq::<bool>(publ.unwrap_or(true).into()))
                .order(reviews::created_at.desc())
                .limit(limit.unwrap_or(8).into())
                .offset(offset.unwrap_or(0).into())
                .load::<Review>(c)
        }).await
    }

    pub async fn insert(review: NewReview, conn: &DbConn) -> QueryResult<Review> {
        conn.run(move |c| {
            diesel::insert_into(all_reviews)
                .values(&review)
                .get_result(c)
        }).await
    }

    pub async fn publish_with_id(id: i32, conn: &DbConn) -> QueryResult<Review> {
        conn.run(move |c| {
            diesel::update(all_reviews.find(id))
                .set(reviews::published.eq(true))
                .get_result(c)
        }).await
    }

    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(all_reviews)
                .filter(reviews::id.eq(id))
                .execute(c)
        }).await
    }

    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(all_reviews).execute(c)).await
    }
}

#[get("/<id>")]
pub async fn get(conn: DbConn, id: i32) -> Result<Json<Review>, NotFound<Json<ApiError>>> {
    Review::get_with_id(id, &conn)
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details:e.to_string(),
            }))
        })
}


#[get("/?<limit>&<offset>&<published>")]
pub async fn get_all(
    conn: DbConn,
    limit: Option<u8>,
    offset: Option<u32>,
    published: Option<bool>
) -> Result<Json<Vec<Review>>, Json<ApiError>>
{
    Review::get_all(limit, offset, published, &conn)
        .await
        .map(Json)
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })
}

#[post("/", format = "json", data = "<review>")]
pub async fn new(conn: DbConn, review: Json<NewReview>) -> Result<Created<Json<Review>>, Json<ApiError>> {
    Review::insert(review.into_inner(), &conn)
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })
}

#[patch("/<id>")]
pub async fn publish(conn: DbConn, id: i32) -> Result<Json<Review>, NotFound<Json<ApiError>>> {
    Review::publish_with_id(id, &conn)
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError { details: e.to_string() }))
        })
}

#[delete("/<id>")]
pub async fn destroy(conn: DbConn, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    Review::delete_with_id(id, &conn) 
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError { details: e.to_string() }))
        })
}
