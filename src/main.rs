use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use config::{Config, ConfigError, File};
use handlebars::Handlebars;
use reqwest;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct Settings {
    base_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("Config.toml"))
            .build()?;

        s.try_deserialize()
    }
}

async fn scrape_page_for_main(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    let parsed_html = Html::parse_document(&resp);
    let selector = &Selector::parse("main").unwrap();
    let main_el = parsed_html
        .select(selector)
        .next()
        .map(|el| el.html())
        .unwrap_or(String::from("<div>something went wrong</div>"));
    Ok(main_el)
}

#[get("/{path:.*}")]
async fn get_handler(
    hb: web::Data<Handlebars<'_>>,
    config: web::Data<Settings>,
    path: web::Path<String>,
) -> impl Responder {
    let url = config.base_url.clone() + path.as_str();
    let main_html = scrape_page_for_main(&url).await.unwrap();
    let data = json!({
        "url": config.base_url.clone() + path.as_str(),
        "main": main_html
    });

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

    let config = Settings::new().unwrap();
    let config_ref = web::Data::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .app_data(config_ref.clone())
            .service(get_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
