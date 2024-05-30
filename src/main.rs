//add the modules
mod api;
mod models;
mod repository;
use actix_web::{
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use api::user_api::{create_user, delete_user, get_users, index};
use repository::mongodb_repo::MongoRepo;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_users)
            .service(create_user)
            .service(server_ping)
            .service(delete_user)
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn server_ping() -> HttpResponse {
    return HttpResponse::Ok().body("Working Fine !");
}
