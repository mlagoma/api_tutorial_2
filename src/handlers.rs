use super::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder};
use diesel::prelude::*;

use crate::models::{NewTweet, Tweet, TweetPayload};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/tweets")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let tweets = web::block(move || {
    let conn = pool.get()?;
    find_all(&conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(tweets))
}

#[post("/tweets")]
async fn create(
  pool: web::Data<DbPool>,
  payload: web::Json<TweetPayload>,
) -> Result<HttpResponse, Error> {
  let tweet = web::block(move || {
    let conn = pool.get()?;
    add_a_tweet(&payload.message, &conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(tweet))
}

#[get("/tweets/{id}")]
async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let tweet = web::block(move || {
    let conn = pool.get()?;
    find_by_id(id.into_inner(), &conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(tweet))
}

#[put("/tweets/{id}")]
async fn update(id: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().body(format!("Tweet#edit {}", id))
}

#[delete("/tweets/{id}")]
async fn destroy(id: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().body(format!("Tweet#delete {}", id))
}

fn add_a_tweet(_message: &str, conn: &PgConnection) -> Result<Tweet, DbError> {
  use crate::schema::tweets::dsl::*;

  let new_tweet = NewTweet {
    message: _message,
    created_at: chrono::Local::now().naive_local(),
  };

  let res = diesel::insert_into(tweets)
    .values(&new_tweet)
    .get_result(conn)?;
  Ok(res)
}

fn find_all(conn: &PgConnection) -> Result<Vec<Tweet>, DbError> {
  use crate::schema::tweets::dsl::*;

  let items = tweets.load::<Tweet>(conn)?;
  Ok(items)
}

fn find_by_id(tweet_id: i32, conn: &PgConnection) -> Result<Option<Tweet>, DbError> {
  use crate::schema::tweets::dsl::*;

  let tweet = tweets
    .filter(id.eq(tweet_id))
    .first::<Tweet>(conn)
    .optional()?;

  Ok(tweet)
}
