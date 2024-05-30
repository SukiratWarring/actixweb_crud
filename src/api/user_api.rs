use std::string;

use crate::{
    models::user_model::{self, USER},
    repository::mongodb_repo::MongoRepo,
};
use actix::{fut::future::result, Actor, StreamHandler};
use actix_web::{
    get,
    http::Error,
    post,
    web::{self, Data, Json, Path},
    HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use mongodb::bson::oid::ObjectId;

#[get("/getuser/{id}")]
async fn get_users(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let result: Result<USER, String> = db.get_user_handler(&id).await;
    // return HttpResponse::Ok().json(result);
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[post("/createuser")]
async fn create_user(db: Data<MongoRepo>, new_user: Json<USER>) -> HttpResponse {
    let user = USER {
        id: None,
        name: new_user.name.clone(),
        location: new_user.location.clone(),
        message: new_user.message.clone(),
    };
    let result = db.create_user_handler(user).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/deleteuser/{id}")]
async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id: String = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    match db.delete_user_handler(&id).await {
        Ok(r) => return HttpResponse::Ok().json(r),
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };
}

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let resp: Result<HttpResponse, actix_web::Error> = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}
