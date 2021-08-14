use crate::app::module::api_auth::{dao::APIAuthDAO, entity::APIAuthInfo};
use actix_web::{web, HttpResponse};
use mongodb::Client;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use uuid::Uuid;
use log::error;

const SERVICE_NAME: &str = "APIAuthService";

fn get_serv_name() -> &'static str {
    SERVICE_NAME
}

pub struct APIAuthService {}

impl APIAuthService {
    pub async fn get_by_id(&self, mg_cli: web::Data<Client>, dto: APIAuthInfo) -> HttpResponse {
        let dao = APIAuthDAO {
            client: mg_cli.clone(),
        };

        // verify dto
        if let Some(id) = dto.id {
            let res = dao.find_by_id(&id).await;
            match res {
                Ok(info) => match info {
                    Some(info) => {
                        return HttpResponse::Ok()
                            .content_type("application/json")
                            .json(info)
                    }
                    None => {
                        return HttpResponse::NotFound()
                            .content_type("plain/text")
                            .body("no such id")
                    }
                },
                Err(err) => {
                    error!("[{}] failed to get by id {}, detail: {}", get_serv_name(), &id, err);
                    return HttpResponse::InternalServerError()
                        .content_type("plain/text")
                        .body(err.to_string());
                }
            }
        } else {
            return HttpResponse::UnprocessableEntity()
                .content_type("plain/text")
                .body("no `_id` field received");
        }
    }

    pub async fn get_by_cond(&self, mg_cli: web::Data<Client>, dto: APIAuthInfo) -> HttpResponse {
        let dao = APIAuthDAO {
            client: mg_cli.clone(),
        };

        let res = dao.find_all(&dto).await;
        match res {
            Ok(infos) => {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .json(infos)
            }
            Err(err) => {
                error!("[{}] failed to query by cond: {}, detail: {}", get_serv_name(), serde_json::to_string(&dto).unwrap(), err);
                return HttpResponse::InternalServerError()
                    .content_type("plain/text")
                    .body(err.to_string());
            }
        }
    }

    pub async fn create(&self, mg_cli: web::Data<Client>, dto: APIAuthInfo) -> HttpResponse {
        // verify necessary fields
        if dto.app == None || dto.api == None || dto.expire == None || dto.uid == None {
            return HttpResponse::UnprocessableEntity()
                .content_type("plain/text")
                .body("require fields: `APP/API/EXPIRE/UID`");
        }

        // generate ak, sk
        let ak = Uuid::new_v4().to_simple().to_string();
        let sk: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let mut to_create = dto.clone();
        to_create.ak = Some(ak);
        to_create.sk = Some(sk);

        let dao = APIAuthDAO {
            client: mg_cli.clone(),
        };
        let res = dao.insert_one(&to_create).await;
        match res {
            Ok(info) => {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .json(info)
            }
            Err(err) => {
                error!("[{}] failed to create: {}, detail: {}", get_serv_name(), serde_json::to_string(&to_create).unwrap(), err);
                return HttpResponse::InternalServerError()
                    .content_type("plain/text")
                    .body(err.to_string());
            }
        }
    }

    pub async fn update_by_id(&self, mg_cli: web::Data<Client>, dto: APIAuthInfo) -> HttpResponse {
        if dto.id == None {
            return HttpResponse::UnprocessableEntity()
                .content_type("plain/text")
                .body("require fields: `_id`");
        }

        let dao = APIAuthDAO {
            client: mg_cli.clone(),
        };
        let res = dao.update_by_id(&dto).await;
        match res {
            Ok(info) => {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .json(info)
            }
            Err(err) => {
                error!("[{}] failed to update: {}, detail: {}", get_serv_name(), serde_json::to_string(&dto).unwrap(), err);
                return HttpResponse::InternalServerError()
                    .content_type("plain/text")
                    .body(err.to_string());
            }
        }
    }

    pub async fn delete_by_id(&self, mg_cli: web::Data<Client>, dto: APIAuthInfo) -> HttpResponse {
        if dto.id == None {
            return HttpResponse::UnprocessableEntity()
                .content_type("plain/text")
                .body("require fields: `_id`");
        }

        let dao = APIAuthDAO {
            client: mg_cli.clone(),
        };
        let id = dto.id.unwrap();
        let res = dao.delete_one(&id).await;
        match res {
            Ok(_) => {
                return HttpResponse::NoContent()
                    .content_type("plain/text")
                    .body("");
            }
            Err(err) => {
                error!("[{}] failed to delete by id {}, detail: {}", get_serv_name(), id, err);
                return HttpResponse::InternalServerError()
                    .content_type("plain/text")
                    .body(err.to_string());
            }
        }
    }
}
