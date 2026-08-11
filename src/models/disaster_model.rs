use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// 'pending', 'new', 'aid_dispatched', 'aid_arrived', 'resolved'

pub enum DisasterStatus {
    Pending,
    New,
    AidDispatched,
    AidArrived,
    Resolved,
}

impl DisasterStatus {
    pub fn to_string(self) -> String {
        match self {
            DisasterStatus::AidArrived => "aid_arrived".to_string(),
            DisasterStatus::Pending => "pending".to_string(),
            DisasterStatus::New => "new".to_string(),
            DisasterStatus::AidDispatched => "aid_dispatched".to_string(),
            DisasterStatus::Resolved => "resolved".to_string(),
        }
    }
}

use crate::models::disaster_report_image_model::DisasterImagePayload;

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
    pub is_anon: Option<bool>,
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
    pub is_anon: Option<bool>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDisasterReportWithImage {
    pub title: String,
    pub description: Option<String>,
    pub street: Option<String>,
    pub city: String,
    pub lat: f64,
    pub lng: f64,
    pub is_anon: Option<bool>,
    pub attachment: Vec<DisasterImagePayload>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDisasterReportPayload {
    pub title: String,
    pub description: Option<String>,
    pub street: Option<String>,
    pub city: String,
    pub lat: f64,
    pub lng: f64,
    pub is_anon: Option<bool>,
    pub images: Vec<DisasterImagePayload>,
}

impl CreateDisasterReportWithImage {
    pub fn to_record(&self, user_id: Uuid) -> CreateDisasterReport {
        CreateDisasterReport {
            user_id,
            title: self.title.clone(),
            description: self.description.clone(),
            street: self.street.clone(),
            city: self.city.clone(),
            lat: self.lat,
            lng: self.lng,
            is_anon: self.is_anon,
            status: self
                .status
                .clone()
                .unwrap_or(DisasterStatus::Pending.to_string()),
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::disaster_report_images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DisasterReportImageModel {
    pub id: Uuid,
    pub disaster_report_id: Uuid,
    pub url: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::disaster_report_images)]
pub struct CreateDisasterReportImage {
    pub disaster_report_id: Uuid,
    pub url: String,
}
