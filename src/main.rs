use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct QueryData {
    content: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct PostData {
    email: String,
}

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

async fn url_query(query: web::Query<QueryData>) -> impl Responder {
    let _client = Client::new();

    match &query.content {
        Some(content) if !content.is_empty() => {
            let my_query = format!("You included: {} ", content);
            HttpResponse::Ok()
                .content_type("text/html")
                .body(my_query)
        }
        _ => HttpResponse::Ok()
            .content_type("text/html")
            .body("No content query received!"),
    }
}

async fn url_post(data: web::Json<PostData>) -> impl Responder {
    HttpResponse::Ok().json(data.into_inner()) // Echos back the data received
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_method()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("https://your-ngrok-address.ngrok-free.app")
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/hello", web::get().to(hello_world))
            .route("/query", web::get().to(url_query))
            .route("/capture", web::post().to(url_post)) 
   })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
