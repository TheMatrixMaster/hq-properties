use diesel::prelude::*;
use diesel::result::QueryResult;

use std::str::FromStr;
use chrono::NaiveDateTime;

use crate::schema::{listings, listing_images};
use crate::schema::listings::dsl::listings as all_listings;
use crate::schema::listing_images::dsl::listing_images as all_listing_imgs;

use rocket::{get, delete, Responder};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::response::status::{NotFound, NoContent, BadRequest};

use super::{DbConn, ApiError};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::MarketStatus"]
pub enum MarketStatus {
    Sold,
    Sale,
    Rent,
    Expired
}

impl FromStr for MarketStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sold" => Ok(MarketStatus::Sold),
            "sale" => Ok(MarketStatus::Sale),
            "rent" => Ok(MarketStatus::Rent),
            "expired" => Ok(MarketStatus::Expired),
            _ => Err(format!("Unrecognized market status: {}", s)),
        }
    }
}

#[derive(Debug, Responder)]
pub enum StatusErr<T> {
    BadRequest(BadRequest<T>),
    NotFound(NotFound<T>),
}

#[derive(Identifiable, Queryable, Selectable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = listings)]
pub struct Listing {
    pub id: i32,
    pub city: String,
    pub address: String,
    pub listing_url: String,
    pub bedrooms: i16,
    pub bathrooms: i16,
    pub area: f64,
    pub price: i32,
    pub market_st: MarketStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = listings)]
pub struct NewListing {
    pub id: i32,
    pub city: String,
    pub address: String,
    pub listing_url: String,
    pub bedrooms: i16,
    pub bathrooms: i16,
    pub area: f64,
    pub price: i32,
    pub market_st: MarketStatus,
    pub updated_at: NaiveDateTime
}

#[derive(Identifiable, Queryable, Selectable, Associations, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Listing))]
#[diesel(table_name = listing_images)]
pub struct ListingImage {
    pub id: i32,
    pub listing_id: i32,
    pub url: String,
    pub priority: i16,
    pub tag: Option<String>,
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = listing_images)]
pub struct NewListingImage {
    pub id: i32,
    pub listing_id: i32,
    pub url: String,
    pub priority: i16,
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FullListing {
    pub listing: Listing,
    pub imgs: Vec<ListingImage>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FullListingReturnPayload {
    pub data: Vec<FullListing>,
    pub size: i64,
}

impl ListingImage {
    pub async fn get_with_id(id: i32, conn: &DbConn) -> QueryResult<ListingImage> {
        conn.run(move |c| {
            all_listing_imgs.find(id).first::<ListingImage>(c)
        }).await
    }

    pub async fn get_with_listing_id(listing_id: i32, conn: &DbConn) -> QueryResult<Vec<ListingImage>> {
        conn.run(move |c| {
            let listing = all_listings.find(listing_id).first::<Listing>(c)?;
            ListingImage::belonging_to(&listing)
                .order(listing_images::priority.asc())
                .load::<ListingImage>(c)
        }).await
    }

    pub async fn get_with_listing_ids(ls: Vec<Listing>, conn: &DbConn) -> QueryResult<Vec<FullListing>> {
        conn.run(move |c| {
            match ListingImage::belonging_to(&ls)
                    .order(listing_images::priority.asc())
                    .load::<ListingImage>(c)
            {
                Ok(imgs) => Ok(
                    imgs.grouped_by(&ls)
                        .into_iter()
                        .zip(ls)
                        .map(|(imgs, listing)| FullListing { listing, imgs })
                        .collect::<Vec<_>>()
                ),
                Err(e) => Err(e)
            }
        }).await
    }

    pub async fn insert(img: NewListingImage, conn: &DbConn) -> QueryResult<ListingImage> {
        conn.run(move |c| {
            diesel::insert_into(all_listing_imgs)
                .values(&img)
                .get_result(c)
        }).await
    }

    pub async fn insert_many(imgs: Vec<NewListingImage>, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::insert_into(all_listing_imgs)
                .values(&imgs)
                .execute(c)
        }).await
    }

    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(all_listing_imgs)
                .filter(listing_images::id.eq(id))
                .execute(c)
        }).await
    }

    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(all_listing_imgs).execute(c)).await
    }
}

impl Listing {
    pub async fn get_with_id(id: i32, conn: &DbConn) -> QueryResult<Listing> {
        conn.run(move |c| {
            all_listings.find(id).first::<Listing>(c)
        }).await
    }

    pub async fn get_all(
        limit: Option<u8>,
        offset: Option<u32>,
        status: Option<MarketStatus>,
        conn: &DbConn
    ) -> QueryResult<(Vec<Listing>, i64)> 
    {
        conn.run(move |c| {
            let mut query1 = all_listings.into_boxed();
            let mut query2 = all_listings.into_boxed();
            
            if let Some(status) = status  {
                query1 = query1.filter(listings::market_st.eq::<MarketStatus>(status));
                query2 = query2.filter(listings::market_st.eq::<MarketStatus>(status));
            }
            let size = query1
                .count()
                .get_result::<i64>(c)?;
            let ls = query2
                .order(listings::updated_at.desc())
                .limit(limit.unwrap_or(8).into())
                .offset(offset.unwrap_or(0).into())
                .load::<Listing>(c)?;

            Ok((ls, size))
        }).await
    }

    pub async fn insert(listing: NewListing, conn: &DbConn) -> QueryResult<Listing> {
        conn.run(move |c| {
            diesel::insert_into(all_listings)
                .values(&listing)
                .get_result(c)
        }).await
    }

    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(all_listings)
                .filter(listings::id.eq(id))
                .execute(c)
        }).await
    }

    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(all_listings).execute(c)).await
    }
}


#[get("/<id>")]
pub async fn get(conn: DbConn, id: i32) -> Result<Json<FullListing>, NotFound<Json<ApiError>>> {
    let listing = Listing::get_with_id(id, &conn)
        .await
        .map_err(|e| {
            NotFound(Json(ApiError {
                details:e.to_string(),
            }))
        })?;
    
    let imgs = ListingImage::get_with_listing_id(id, &conn)
        .await
        .map_err(|e| {
            NotFound(Json(ApiError {
                details:e.to_string(),
            }))
        })?;

    Ok(Json(FullListing { listing, imgs }))
}

#[get("/?<limit>&<offset>&<market_st>")]
pub async fn get_all(
    conn: DbConn,
    limit: Option<u8>,
    offset: Option<u32>,
    market_st: Option<String>
) -> Result<Json<FullListingReturnPayload>, StatusErr<Json<ApiError>>>
{
    let m = market_st
        .map(|v| MarketStatus::from_str(&v))
        .transpose()
        .map_err(|e| {
            StatusErr::BadRequest(BadRequest(Json(ApiError {
                details:e.to_string(),
            })))
        })?;

    let (ls, size) = Listing::get_all(limit, offset, m, &conn)
        .await
        .map_err(|e| {
            StatusErr::NotFound(NotFound(Json(ApiError {
                details:e.to_string(),
            })))
        })?;

    let data = ListingImage::get_with_listing_ids(ls, &conn)
        .await
        .map_err(|e| {
            StatusErr::NotFound(NotFound(Json(ApiError {
                details:e.to_string(),
            })))
        })?;

    Ok(Json(FullListingReturnPayload { data, size }))
}

#[delete("/<id>")]
pub async fn destroy(conn: DbConn, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    Listing::delete_with_id(id, &conn) 
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError { details: e.to_string() }))
        })
}

