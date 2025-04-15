use actix_files::Files; // Для работы с статическими файлами
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use askama::Template;
use dotenvy::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct CryptoCompareResponse {
    Data: Vec<ArticleData>,
}

#[derive(Deserialize, Debug)]
struct ArticleData {
    title: String,
    url: String,
    source: String,
    published_on: u64,
    body: String,
}

#[derive(Clone, Debug, serde::Serialize)]
struct Article {
    title: String,
    url: String,
    source: String,
    date: String,
    summary: String,
}

fn unix_to_date(timestamp: u64) -> String {
    let naive = chrono::NaiveDateTime::from_timestamp_millis((timestamp * 1000) as i64).unwrap_or_default();
    naive.format("%Y-%m-%d %H:%M:%S").to_string()
}

async fn index() -> impl Responder {
    let template = IndexTemplate { articles: Vec::new() };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

async fn search(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let search_term = query.get("query").unwrap_or(&"BTC".to_string()).to_uppercase();
    let api_key = env::var("CRYPTOCOMPARE_API_KEY").unwrap_or_default();

    let url = format!(
        "https://min-api.cryptocompare.com/data/v2/news/?categories={}&api_key={}",
        search_term, api_key
    );

    let client = Client::new();
    let resp = client.get(&url).send().await;

    let mut articles = Vec::new();

    if let Ok(response) = resp {
        if let Ok(json) = response.json::<CryptoCompareResponse>().await {
            for data in json.Data.iter().take(10) {
                articles.push(Article {
                    title: data.title.clone(),
                    url: data.url.clone(),
                    source: data.source.clone(),
                    date: unix_to_date(data.published_on),
                    summary: data.body.chars().take(200).collect(),
                });
            }
        }
    }

    let template = IndexTemplate { articles };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/search", web::get().to(search))
            // Добавляем сервис для статических файлов
            .service(Files::new("/static", "./static").show_files_listing()) 
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
