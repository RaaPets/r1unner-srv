mod app_state;
mod config;
mod liba;
mod web_app;

use app_state::AppState;

use actix_web::{web, HttpServer, App};

//  //  //  //  //  //  //  //
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::parse_cli();
    println!("r1unner: {}:{}", config.bind, config.port);

    let state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        use web_app::*;
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
