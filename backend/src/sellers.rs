use diesel::prelude::*;
use diesel::result::QueryResult;

use chrono::NaiveDateTime;

use crate::schema::sellers;
use crate::schema::sellers::dsl::sellers as all_sellers;

use rocket::{get, post, delete};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::status::{Created, NoContent, NotFound};

use super::{DbConn, ApiError};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = sellers)]
pub struct Seller {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: String,
    pub postal_code: Option<String>,
    pub city: String,
    pub sell_period: String,
    pub other: Option<String>,
    pub created_at: NaiveDateTime,        
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = sellers)]
pub struct NewSeller {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: String,
    pub postal_code: Option<String>,
    pub city: String,
    pub sell_period: String,
    pub other: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SellerReturnPayload {
    pub data: Vec<Seller>,
    pub size: i64,
}

impl Seller {
    pub async fn get_with_id(id: i32, conn: &DbConn) -> QueryResult<Seller> {
        conn.run(move |c| {
            all_sellers.find(id)
                .first::<Seller>(c)
        }).await
    }

    pub async fn get_size(conn: &DbConn) -> QueryResult<i64> {
        conn.run(move |c| {
            all_sellers
                .count()
                .get_result::<i64>(c)
        }).await
    }

    pub async fn get_all(
        limit: Option<u8>,
        offset: Option<u32>,
        conn: &DbConn
    ) -> QueryResult<Vec<Seller>>
    {
        conn.run(move |c| {
            all_sellers
                .order(sellers::created_at.desc())
                .limit(limit.unwrap_or(8).into())
                .offset(offset.unwrap_or(0).into())
                .load::<Seller>(c)
        }).await
    }

    pub async fn insert(seller: NewSeller, conn: &DbConn) -> QueryResult<Seller> {
        conn.run(move |c| {
            diesel::insert_into(all_sellers)
                .values(&seller)
                .get_result(c)
        }).await
    }

    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(all_sellers)
                .filter(sellers::id.eq(id))
                .execute(c)
        }).await
    }

    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(all_sellers).execute(c)).await
    }
}

#[get("/<id>")]
pub async fn get(conn: DbConn, id: i32) -> Result<Json<Seller>, NotFound<Json<ApiError>>> {
    Seller::get_with_id(id, &conn)
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details:e.to_string(),
            }))
        })
}

#[get("/?<limit>&<offset>")]
pub async fn get_all(
    conn: DbConn,
    limit: Option<u8>,
    offset: Option<u32>,
) -> Result<Json<SellerReturnPayload>, Json<ApiError>>
{
    let size = Seller::get_size(&conn)
        .await
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })?;

    let data = Seller::get_all(limit, offset, &conn)
        .await
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })?;

    Ok(Json(SellerReturnPayload { data, size }))
}

#[post("/", format = "json", data = "<seller>")]
pub async fn new(conn: DbConn, seller: Json<NewSeller>) -> Result<Created<Json<Seller>>, Json<ApiError>> {
    Seller::insert(seller.into_inner(), &conn)
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })
}

#[delete("/<id>")]
pub async fn destroy(conn: DbConn, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    Seller::delete_with_id(id, &conn) 
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError { details: e.to_string() }))
        })
}
