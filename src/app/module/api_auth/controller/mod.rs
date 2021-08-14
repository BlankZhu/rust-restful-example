use crate::app::module::api_auth::{entity::APIAuthInfo, service::APIAuthService};
use actix_web::{web, HttpResponse};
use mongodb::Client;

pub fn api_auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api-auth")
            .route(web::get().to(query))
            .route(web::post().to(create))
            .route(web::put().to(update_by_id))
            .route(web::delete().to(delete_by_id)),
    );
}

// get by id via query params
async fn query(
    mg_cli: web::Data<Client>,
    web::Query(info): web::Query<APIAuthInfo>,
) -> HttpResponse {
    let serv = APIAuthService {};

    match info.id {
        Some(_) => {
            return serv.get_by_id(mg_cli, info).await;
        }
        None => {
            return serv.get_by_cond(mg_cli, info).await;
        }
    }
}

// create by json entity
async fn create(mg_cli: web::Data<Client>, info: web::Json<APIAuthInfo>) -> HttpResponse {
    let serv = APIAuthService {};
    return serv.create(mg_cli, info.0).await;
}

// update by id via json entity
async fn update_by_id(mg_cli: web::Data<Client>, info: web::Json<APIAuthInfo>) -> HttpResponse {
    let serv = APIAuthService {};
    return serv.update_by_id(mg_cli, info.0).await;
}

// delete by id via query params
async fn delete_by_id(
    mg_cli: web::Data<Client>,
    web::Query(info): web::Query<APIAuthInfo>,
) -> HttpResponse {
    let serv = APIAuthService {};
    return serv.delete_by_id(mg_cli, info).await;
}
