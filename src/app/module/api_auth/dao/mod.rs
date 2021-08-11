use crate::app::{constants, module::api_auth::entity::APIAuthInfo};
use actix_web::web;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Error,
    options::FindOptions,
    Client,
};

pub struct APIAuthDAO {
    pub client: web::Data<Client>,
}

impl APIAuthDAO {
    pub async fn find_by_id(&self, id: &ObjectId) -> Result<Option<APIAuthInfo>, Error> {
        let db = self.client.database(constants::database::AUTH_DB);
        let coll = db.collection::<APIAuthInfo>(constants::collection::COLL_API_AUTH);
        let filter = doc! { "_id": id };
        coll.find_one(filter, None).await
    }

    pub async fn find_all(&self, cond: &APIAuthInfo) -> Result<Vec<APIAuthInfo>, Error> {
        let db = self.client.database(constants::database::AUTH_DB);
        let coll = db.collection::<APIAuthInfo>(constants::collection::COLL_API_AUTH);
        let find_opts = FindOptions::builder().sort(doc! {"_id": 1}).build();

        let mut doc = Document::new();
        if let Some(id) = cond.id {
            doc.insert("_id", id);
        }
        if let Some(ak) = &cond.ak {
            doc.insert("AK", ak);
        }
        if let Some(app) = &cond.app {
            doc.insert("APP", app);
        }
        if let Some(api) = &cond.api {
            doc.insert("API", api);
        }
        if let Some(expire) = &cond.expire {
            let dt = expire.clone();
            let ch_dt: chrono::DateTime<Utc> = dt.into();
            doc.insert("EXPIRE", ch_dt);
        }
        if let Some(sk) = &cond.sk {
            doc.insert("SK", sk);
        }
        if let Some(uid) = &cond.uid {
            doc.insert("UID", uid);
        }

        let mut cur = coll.find(doc, find_opts).await?;
        let mut ret: Vec<APIAuthInfo> = Vec::new();
        while let Some(info) = cur.next().await {
            match info {
                Ok(info) => ret.push(info),
                Err(err) => return Err(err),
            }
        }
        Ok(ret)
    }

    pub async fn insert_one(&self, entity: &APIAuthInfo) -> Result<APIAuthInfo, Error> {
        let db = self.client.database(constants::database::AUTH_DB);
        let coll = db.collection(constants::collection::COLL_API_AUTH);

        let expire: DateTime<Utc> = entity.expire.unwrap().into();
        let doc = doc! {
            "AK": entity.ak.as_ref().unwrap(),
            "APP": entity.app.as_ref().unwrap(),
            "API": entity.api.as_ref().unwrap(),
            "EXPIRE": expire,
            "SK": entity.sk.as_ref().unwrap(),
            "UID": entity.uid.as_ref().unwrap(),
        };
        
        coll.insert_one(doc, None).await.and_then(|res| {
            let id = res.inserted_id;
            let mut ret = entity.clone();
            ret.id = id.as_object_id();
            return Ok(ret);
        })
    }

    pub async fn update_by_id(&self, entity: &APIAuthInfo) -> Result<APIAuthInfo, Error> {
        let db = self.client.database(constants::database::AUTH_DB);
        let coll = db.collection::<APIAuthInfo>(constants::collection::COLL_API_AUTH);

        let ret = entity.clone();

        let id = entity.id.unwrap();
        let query = doc! { "_id": id };

        let mut update = Document::new();
        if let Some(ak) = &entity.ak {
            update.insert("AK", &ak);
        }
        if let Some(app) = &entity.app {
            update.insert("APP", &app);
        }
        if let Some(api) = &entity.api {
            update.insert("API", &api);
        }
        if let Some(expire) = &entity.expire {
            let dt = expire.clone();
            let ch_dt: chrono::DateTime<Utc> = dt.into();
            update.insert("EXPIRE", ch_dt);
        }
        if let Some(sk) = &entity.sk {
            update.insert("SK", &sk);
        }
        if let Some(uid) = &entity.uid {
            update.insert("UID", &uid);
        }
        
        coll.update_one(
            query,
            doc! {
                "$set": update,
            },
            None,
        )
        .await
        .and_then(|_| Ok(ret))
    }

    pub async fn delete_one(&self, id: &ObjectId) -> Result<(), Error> {
        let db = self.client.database(constants::database::AUTH_DB);
        let coll = db.collection::<APIAuthInfo>(constants::collection::COLL_API_AUTH);
        
        coll.delete_one(
            doc! {
                "_id" : id
            },
            None,
        )
        .await
        .and_then(|_| Ok(()))
    }
}
