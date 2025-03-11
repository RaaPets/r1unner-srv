use crate::AppState;
use actix_web::{web, delete, get, post, Responder};
use actix_web::http::StatusCode;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
#[derive(serde::Deserialize)]
struct ItemID {
    id: Option<usize>,
}

//  //  //  //  //  //  //  //
#[get("/tasks")]
async fn get_list(state: web::Data<AppState>, query: web::Query<ItemID>) -> impl Responder {
    println!("get list {:?}", query.id);
    let runner = state.runner.read().unwrap();
    runner.get_list()
}

#[get("/task/{id}")]
async fn get_item(state: web::Data<AppState>, path: web::Path<usize>) -> (impl Responder, StatusCode) {
    let id = path.into_inner();
    println!("get item {}", id);
    let runner = state.runner.read().unwrap();
    let res = runner.get_via_id(id);
    match res {
        Some(s) => (Some(s), StatusCode::OK),
        None => (None, StatusCode::BAD_REQUEST),
    }
}

#[post("/")]
async fn insert_item(state: web::Data<AppState>, info: String) -> (impl Responder, StatusCode) {
    println!("insert item\n'{}'", info);
    let mut runner = state.runner.write().unwrap();
    let new_id = runner.insert_via_id_info(&info);
    (format!("{}", new_id), StatusCode::CREATED)
}

#[delete("/task/{id}")]
async fn delete_item(state: web::Data<AppState>, path: web::Path<usize>) -> (impl Responder, StatusCode) {
    let id = path.into_inner();
    println!("DELETE item {}", id);
    let mut runner = state.runner.write().unwrap();
    let res = runner.remove_via_id(id);
    match res {
        Some(s) => (Some(s), StatusCode::OK),
        None => (None, StatusCode::BAD_REQUEST),
    }
}
