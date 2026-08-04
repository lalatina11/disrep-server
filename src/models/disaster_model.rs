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
    pub lat: f64,
    pub lng: f64,
    pub image: String,
    pub image_storage_url: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
