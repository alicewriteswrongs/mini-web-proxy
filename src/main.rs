use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use scraper::{Html, Selector};
use serde_derive::Serialize;
use serde_json::json;
use std::env;

#[derive(Serialize)]
struct ScrapeInfo {
    main: String,
    url: String,
}

async fn scrape_page_for_main(url: &str) -> Result<ScrapeInfo, Box<dyn std::error::Error>> {
    let text = reqwest::get(url).await?.text().await?;
    let parsed_html = Html::parse_document(&text);
    let selector = &Selector::parse("main").unwrap();
    let main_el = parsed_html
        .select(selector)
        .next()
        .map(|el| el.html())
        .map(|html| ammonia::clean(&html))
        .unwrap_or_else(|| String::from("<div>something went wrong</div>"));

    Ok(ScrapeInfo {
        main: main_el,
        url: String::from(url),
    })
}

#[get("/{path:.*}")]
async fn get_handler(
    hb: web::Data<Handlebars<'_>>,
    baseurl: web::Data<String>,
    path: web::Path<String>,
) -> impl Responder {
    let url = baseurl.get_ref().clone() + path.as_str();
    let scrape_info = scrape_page_for_main(&url).await.unwrap();
    let data = json!(scrape_info);
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let baseurl = env::var("BASE_URL").unwrap();
    let baseurl_ref = web::Data::new(baseurl);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .app_data(baseurl_ref.clone())
            .service(get_handler)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
