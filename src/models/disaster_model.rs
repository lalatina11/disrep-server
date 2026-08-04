use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::disaster_reports)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct DisasterReportsModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub street: Option<String>,
    pub city: String,
    pub lat: f64,
    pub lng: f64,
    pub image: String,
    pub image_storage_url: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::disaster_reports)]
pub struct CreateDisasterReport {
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub street: Option<String>,
    pub city: String,
    pub lat: f64,
    pub lng: f64,
    pub image: String,
    pub image_storage_url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDisasterReportPayload {
    pub title: String,
    pub description: Option<String>,
    pub street: Option<String>,
    pub city: String,
    pub lat: f64,
    pub lng: f64,
    pub image: String,
    pub image_storage_url: String,
}

impl CreateDisasterReportPayload {
    pub fn into_record(self, user_id: Uuid) -> CreateDisasterReport {
        CreateDisasterReport {
            user_id,
            title: self.title,
            description: self.description,
            street: self.street,
            city: self.city,
            lat: self.lat,
            lng: self.lng,
            image: self.image,
            image_storage_url: self.image_storage_url,
        }
    }
}
