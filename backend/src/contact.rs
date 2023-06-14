use diesel::prelude::*;
use diesel::result::QueryResult;

use chrono::NaiveDateTime;

use crate::schema::contact_messages;
use crate::schema::contact_messages::dsl::contact_messages as all_contact_msg;

use rocket::{get, post, delete};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::status::{Created, NoContent, NotFound};

use super::{DbConn, ApiError};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = contact_messages)]
pub struct ContactMessage {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub body: String,
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = contact_messages)]
pub struct NewContactMessage {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub body: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ContactMsgReturnPayload {
    pub data: Vec<ContactMessage>,
    pub size: i64,
}

impl ContactMessage {
    pub async fn get_with_id(id: i32, conn: &DbConn) -> QueryResult<ContactMessage> {
        conn.run(move |c| {
            all_contact_msg.find(id)
                .first::<ContactMessage>(c)
        }).await
    }

    pub async fn get_size(conn: &DbConn) -> QueryResult<i64> {
        conn.run(move |c| {
            all_contact_msg
                .count()
                .get_result::<i64>(c)
        }).await
    }

    pub async fn get_all(
        limit: Option<u8>,
        offset: Option<u32>,
        conn: &DbConn
    ) -> QueryResult<Vec<ContactMessage>>
    {
        conn.run(move |c| {
            all_contact_msg
                .order(contact_messages::created_at.desc())
                .limit(limit.unwrap_or(8).into())
                .offset(offset.unwrap_or(0).into())
                .load::<ContactMessage>(c)
        }).await
    }

    pub async fn insert(message: NewContactMessage, conn: &DbConn) -> QueryResult<ContactMessage> {
        conn.run(move |c| {
            diesel::insert_into(all_contact_msg)
                .values(&message)
                .get_result(c)
        }).await
    }

    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(all_contact_msg)
                .filter(contact_messages::id.eq(id))
                .execute(c)
        }).await
    }

    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(all_contact_msg).execute(c)).await
    }
}

#[get("/<id>")]
pub async fn get(conn: DbConn, id: i32) -> Result<Json<ContactMessage>, NotFound<Json<ApiError>>> {
    ContactMessage::get_with_id(id, &conn)
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
) -> Result<Json<ContactMsgReturnPayload>, Json<ApiError>>
{
    let size = ContactMessage::get_size(&conn)
        .await
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })?;

    let data = ContactMessage::get_all(limit, offset, &conn)
        .await
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })?;

    Ok(Json(ContactMsgReturnPayload { data, size }))
}

#[post("/", format = "json", data = "<message>")]
pub async fn new(conn: DbConn, message: Json<NewContactMessage>) -> Result<Created<Json<ContactMessage>>, Json<ApiError>> {
    ContactMessage::insert(message.into_inner(), &conn)
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })
}

#[delete("/<id>")]
pub async fn destroy(conn: DbConn, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    ContactMessage::delete_with_id(id, &conn) 
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError { details: e.to_string() }))
        })
}
