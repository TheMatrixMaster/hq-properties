use diesel::prelude::*;
use diesel::result::QueryResult;

use chrono::NaiveDateTime;

use crate::schema::buyers;
use crate::schema::buyers::dsl::buyers as all_buyers;

use rocket::{get, post, delete};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::status::{Created, NoContent, NotFound};

use super::{DbConn, ApiError};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = buyers)]
pub struct Buyer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub buy_period: String,
    pub home_type: String,
    pub bedrooms: String,
    pub location: String,
    pub other: Option<String>,
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = buyers)]
pub struct NewBuyer {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub buy_period: String,
    pub home_type: String,
    pub bedrooms: String,
    pub location: String,
    pub other: Option<String>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BuyerReturnPayload {
    pub data: Vec<Buyer>,
    pub size: i64,
}

impl Buyer {
    pub async fn get_with_id(id: i32, conn: &DbConn) -> QueryResult<Buyer> {
        conn.run(move |c| {
            all_buyers.find(id)
                .first::<Buyer>(c)
        }).await
    }

    pub async fn get_size(conn: &DbConn) -> QueryResult<i64> {
        conn.run(move |c| {
            all_buyers
                .count()
                .get_result::<i64>(c)
        }).await
    }

    pub async fn get_all(
        limit: Option<u8>,
        offset: Option<u32>,
        conn: &DbConn
    ) -> QueryResult<Vec<Buyer>>
    {
        conn.run(move |c| {
            all_buyers
                .order(buyers::created_at.desc())
                .limit(limit.unwrap_or(8).into())
                .offset(offset.unwrap_or(0).into())
                .load::<Buyer>(c)
        }).await
    }

    pub async fn insert(buyer: NewBuyer, conn: &DbConn) -> QueryResult<Buyer> {
        conn.run(move |c| {
            diesel::insert_into(all_buyers)
                .values(&buyer)
                .get_result(c)
        }).await
    }

    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(all_buyers)
                .filter(buyers::id.eq(id))
                .execute(c)
        }).await
    }

    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(all_buyers).execute(c)).await
    }
}

#[get("/<id>")]
pub async fn get(conn: DbConn, id: i32) -> Result<Json<Buyer>, NotFound<Json<ApiError>>> {
    Buyer::get_with_id(id, &conn)
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
) -> Result<Json<BuyerReturnPayload>, Json<ApiError>>
{
    let size = Buyer::get_size(&conn)
        .await
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })?;

    let data = Buyer::get_all(limit, offset, &conn)
        .await
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })?;

    Ok(Json(BuyerReturnPayload { data, size }))
}

#[post("/", format = "json", data = "<buyer>")]
pub async fn new(conn: DbConn, buyer: Json<NewBuyer>) -> Result<Created<Json<Buyer>>, Json<ApiError>> {
    Buyer::insert(buyer.into_inner(), &conn)
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })
}

#[delete("/<id>")]
pub async fn destroy(conn: DbConn, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    Buyer::delete_with_id(id, &conn) 
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError { details: e.to_string() }))
        })
}
