use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[get("/tweets")]
async fn tweet_index() -> impl Responder {
    HttpResponse::Ok().body("Tweet#index")
}

#[post("/tweets")]
async fn tweet_create() -> impl Responder {
    HttpResponse::Ok().body("Tweet#new")
}

#[get("/tweets/{id}")]
async fn tweet_show(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Tweet#show {}", id))
}

#[put("/tweets/{id}")]
async fn tweet_update(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Tweet#edit {}", id))
}

#[delete("/tweets/{id}")]
async fn tweet_destroy(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Tweet#delete {}", id))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .service(tweet_index)
            .service(tweet_create)
            .service(tweet_show)
            .service(tweet_update)
            .service(tweet_destroy)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
