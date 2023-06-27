mod util;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{Pool, Postgres};
use std::env;
use util::create_db_conn_pool;

type DbPool = Pool<Postgres>;

#[cfg(debug_assertions)]
use dotenvy::dotenv;

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("Alive")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/sum")]
async fn query(pool: web::Data<DbPool>) -> impl Responder {
    struct Sum {
        sum: Option<i32>,
    }

    let query = sqlx::query_as!(Sum, "SELECT 5 + 5 as sum")
        .fetch_one(pool.as_ref())
        .await;

    match query {
        Ok(row) => {
            if let Some(sum) = row.sum {
                HttpResponse::Ok().body(format!("Result: {}", sum))
            } else {
                HttpResponse::NotFound().body("Not found")
            }
        }
        Err(_) => HttpResponse::NotFound().body(format!("Not found")),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");
    let db_conn_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let db_conn_pool = create_db_conn_pool(&db_conn_url, 5).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_conn_pool.clone()))
            .service(healthz)
            .service(hello)
            .service(echo)
            .service(query)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
