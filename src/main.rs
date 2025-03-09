mod app_state;
mod config;
mod liba;

use app_state::AppState;

//use actix_web::{get, post, web, Result, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::{delete, get, post, web, App, HttpServer, Responder};

//  //  //  //  //  //  //  //
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::parse_cli();
    println!("r1unner: {}:{}", config.bind, config.port);

    let state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_list)
            .service(get_item)
            .service(delete_item)
            .service(insert_item)
    })
    .bind((config.bind, config.port))?
    .run()
    .await
}

//  //  //  //  //  //  //  //
#[get("/list")]
async fn get_list(state: web::Data<AppState>) -> impl Responder {
    println!("get list");
    let runner = state.runner.read().unwrap();
    runner.get_list()
}

//  //  //  //  //  //  //  //
#[post("/")]
async fn insert_item(state: web::Data<AppState>, info: String) -> impl Responder {
    println!("insert item\n'{}'", info);
    let mut runner = state.runner.write().unwrap();
    let new_id = runner.insert_via_id_info(&info);
    format!("{}", new_id)
}

//  //  //  //  //  //  //  //
#[get("/item")]
async fn get_item(state: web::Data<AppState>, query: web::Query<ItemID>) -> impl Responder {
    println!("get item {}", query.id);
    let runner = state.runner.read().unwrap();
    runner.get_via_id(query.id)
}

#[delete("/item")]
async fn delete_item(state: web::Data<AppState>, query: web::Query<ItemID>) -> impl Responder {
    println!("DELETE item {}", query.id);
    let mut runner = state.runner.write().unwrap();
    runner.remove_via_id(query.id)
}

#[derive(serde::Deserialize)]
struct ItemID {
    id: usize,
}

//  //  //  //  //  //  //  //
