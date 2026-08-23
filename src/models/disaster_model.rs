use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::disaster_report_attachment_model::DisasterAttachmentPayload;

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
            DisasterStatus::New => "new".to_string(),
            DisasterStatus::Pending => "pending".to_string(),
            DisasterStatus::AidDispatched => "aid_dispatched".to_string(),
            DisasterStatus::AidArrived => "aid_arrived".to_string(),
            DisasterStatus::Resolved => "resolved".to_string(),
        }
    }

    pub fn from_str(status: &str) -> Self {
        match status.to_lowercase().as_str() {
            "new" => Self::New,
            "aid_dispatched" => Self::AidDispatched,
            "aid_arrived" => Self::AidArrived,
            "resolved" => Self::Resolved,
            _else => Self::Pending,
        }
    }
}

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

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = crate::schema::disaster_reports)]
pub struct CreateDisasterReport {
    pub user_id: Uuid,
    #[validate(length(
        min = 3,
        max = 255,
        message = "Title must be between 3 and 255 characters"
    ))]
    pub title: String,
    #[validate(length(min = 3, message = "Description must be at least 3 characters"))]
    pub description: Option<String>,
    #[validate(length(min = 3, message = "Street must be at least 3 characters"))]
    pub street: Option<String>,
    #[validate(length(min = 2, message = "City is required"))]
    pub city: String,
    #[validate(range(min = -90.0, max = 90.0, message = "Latitude must be between -90 and 90"))]
    pub lat: f64,
    #[validate(range(min = -180.0, max = 180.0, message = "Longitude must be between -180 and 180"))]
    pub lng: f64,
    pub is_anon: Option<bool>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDisasterReportWithImage {
    #[validate(length(
        min = 3,
        max = 255,
        message = "Title must be between 3 and 255 characters"
    ))]
    pub title: String,
    #[validate(length(min = 3, message = "Description must be at least 3 characters"))]
    pub description: Option<String>,
    #[validate(length(min = 3, message = "Street must be at least 3 characters"))]
    pub street: Option<String>,
    #[validate(length(min = 2, message = "City is required"))]
    pub city: String,
    #[validate(range(min = -90.0, max = 90.0, message = "Latitude must be between -90 and 90"))]
    pub lat: f64,
    #[validate(range(min = -180.0, max = 180.0, message = "Longitude must be between -180 and 180"))]
    pub lng: f64,
    pub is_anon: Option<bool>,
    #[validate(length(min = 1, message = "Please insert an image or video"), nested)]
    pub attachment: Vec<DisasterAttachmentPayload>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDisasterReportPayload {
    #[validate(length(
        min = 3,
        max = 255,
        message = "Title must be between 3 and 255 characters"
    ))]
    pub title: String,
    #[validate(length(min = 3, message = "Description must be at least 3 characters"))]
    pub description: Option<String>,
    #[validate(length(min = 3, message = "Street must be at least 3 characters"))]
    pub street: Option<String>,
    #[validate(length(min = 2, message = "City is required"))]
    pub city: String,
    #[validate(range(min = -90.0, max = 90.0, message = "Latitude must be between -90 and 90"))]
    pub lat: f64,
    #[validate(range(min = -180.0, max = 180.0, message = "Longitude must be between -180 and 180"))]
    pub lng: f64,
    pub is_anon: Option<bool>,
    #[validate(length(min = 1, message = "Please insert an image or video"), nested)]
    pub images: Vec<DisasterAttachmentPayload>,
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
