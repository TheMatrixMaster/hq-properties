use diesel::prelude::*;
use diesel::result::QueryResult;

use chrono::NaiveDateTime;

use crate::schema::videos;
use crate::schema::videos::dsl::videos as all_videos;

use rocket::{get, post, delete};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::response::status::{NotFound, Created, NoContent};

use super::{DbConn, ApiError};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = videos)]
pub struct Video {
    pub id: i32,
    pub video_url: String,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = videos)]
pub struct NewVideo {
    pub video_url: String,
    pub title: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct VideoReturnPayload {
    pub data: Vec<Video>,
    pub size: i64,
}

impl Video {
    pub async fn get_with_id(id: i32, conn: &DbConn) -> QueryResult<Video> {
        conn.run(move |c| {
            all_videos.find(id)
                .first::<Video>(c)
        }).await
    }

    pub async fn get_size(conn: &DbConn) -> QueryResult<i64> {
        conn.run(move |c| {
            all_videos
                .count()
                .get_result::<i64>(c)
        }).await
    }

    pub async fn get_all(
        limit: Option<u8>,
        offset: Option<u32>,
        conn: &DbConn
    ) -> QueryResult<Vec<Video>>
    {
        conn.run(move |c| {
            all_videos
                .order(videos::created_at.desc())
                .limit(limit.unwrap_or(8).into())
                .offset(offset.unwrap_or(0).into())
                .load::<Video>(c)
        }).await
    }

    pub async fn insert(post: NewVideo, conn: &DbConn) -> QueryResult<Video> {
        conn.run(move |c| {
            diesel::insert_into(all_videos)
                .values(&post)
                .get_result(c)
        }).await
    }

    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(all_videos)
                .filter(videos::id.eq(id))
                .execute(c)
        }).await
    }

    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(all_videos).execute(c)).await
    }
}


#[get("/<id>")]
pub async fn get(conn: DbConn, id: i32) -> Result<Json<Video>, NotFound<Json<ApiError>>> {
    Video::get_with_id(id, &conn)
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
) -> Result<Json<VideoReturnPayload>, Json<ApiError>>
{
    let size = Video::get_size(&conn)
        .await
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })?;

    let data = Video::get_all(limit, offset, &conn)
        .await
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })?;

    Ok(Json(VideoReturnPayload { data, size }))
}

#[post("/", format = "json", data = "<post>")]
pub async fn new(conn: DbConn, post: Json<NewVideo>) -> Result<Created<Json<Video>>, Json<ApiError>> {
    Video::insert(post.into_inner(), &conn)
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })
}

#[delete("/<id>")]
pub async fn destroy(conn: DbConn, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    Video::delete_with_id(id, &conn) 
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError { details: e.to_string() }))
        })
}