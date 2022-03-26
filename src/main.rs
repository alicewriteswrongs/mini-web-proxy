use reqwest;
use scraper::{Html, Selector };
use actix_web::{get, App, web, HttpServer, Responder, HttpResponse};

async fn scrape_page_for_main(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    let parsed_html = Html::parse_document(&resp);
    let selector = &Selector::parse("main").unwrap();
    let main_el = parsed_html.select(selector).next().unwrap();
    Ok(main_el.html())
}

#[get("/{path:.*}")]
async fn get_handler(path: web::Path<String>) -> impl Responder {
    let path_str = path.as_str();
    println!("{}", path_str);
    HttpResponse::Ok().body("matched")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
