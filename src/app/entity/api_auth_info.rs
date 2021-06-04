// use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIAuthInfo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "AK", skip_serializing_if = "Option::is_none")]
    pub ak: Option<String>,
    #[serde(rename = "APP", skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(rename = "API", skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[serde(rename = "EXPIRE", skip_serializing_if = "Option::is_none")]
    pub expire: Option<DateTime>,
    #[serde(rename = "SK", skip_serializing_if = "Option::is_none")]
    pub sk: Option<String>,
    #[serde(rename = "UID", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
