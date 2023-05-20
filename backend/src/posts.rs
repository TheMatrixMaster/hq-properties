use diesel::prelude::*;
use diesel::result::QueryResult;

use chrono::NaiveDateTime;

use crate::schema::posts;
use crate::schema::posts::dsl::posts as all_posts;

use rocket::{get, post, delete};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::response::status::{NotFound, NoContent, Created};

use super::{DbConn, ApiError};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub img_url: String,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub img_url: String,
    pub title: String,
    pub body: String,
}

impl Post {
    pub async fn get_with_id(id: i32, conn: &DbConn) -> QueryResult<Post> {
        conn.run(move |c| {
            all_posts.find(id)
                .first::<Post>(c)
        }).await
    }

    pub async fn get_all(
        limit: Option<u8>,
        offset: Option<u32>,
        conn: &DbConn
    ) -> QueryResult<Vec<Post>>
    {
        conn.run(move |c| {
            all_posts
                .order(posts::created_at.desc())
                .limit(limit.unwrap_or(8).into())
                .offset(offset.unwrap_or(0).into())
                .load::<Post>(c)
        }).await
    }

    pub async fn insert(post: NewPost, conn: &DbConn) -> QueryResult<Post> {
        conn.run(move |c| {
            diesel::insert_into(all_posts)
                .values(&post)
                .get_result(c)
        }).await
    }

    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(all_posts)
                .filter(posts::id.eq(id))
                .execute(c)
        }).await
    }

    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(all_posts).execute(c)).await
    }
}

#[get("/<id>")]
pub async fn get(conn: DbConn, id: i32) -> Result<Json<Post>, NotFound<Json<ApiError>>> {
    Post::get_with_id(id, &conn)
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
) -> Result<Json<Vec<Post>>, Json<ApiError>>
{
    Post::get_all(limit, offset, &conn)
        .await
        .map(Json)
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })
}

#[post("/", format = "json", data = "<post>")]
pub async fn new(conn: DbConn, post: Json<NewPost>) -> Result<Created<Json<Post>>, Json<ApiError>> {
    Post::insert(post.into_inner(), &conn)
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError { details: e.to_string() })
        })
}

#[delete("/<id>")]
pub async fn destroy(conn: DbConn, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    Post::delete_with_id(id, &conn) 
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError { details: e.to_string() }))
        })
}
