use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::utils::CommonUtility;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::disaster_report_attachments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DisasterReportAttachmentModel {
    pub id: Uuid,
    pub disaster_report_id: Uuid,
    pub media_url: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::disaster_report_attachments)]
pub struct CreateDisasterReportImage {
    pub disaster_report_id: Uuid,
    pub media_url: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::disaster_report_attachments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DisasterReportsAttachmentModel {
    pub id: Uuid,
    pub disaster_report_id: Uuid,
    pub media_url: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = crate::schema::disaster_report_attachments)]
pub struct DisasterReportsAttachmentModelPayload {
    pub disaster_report_id: Uuid,
    pub media_url: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct DisasterAttachmentPayload {
    #[validate(length(min = 1, message = "Invalid attachment URL"))]
    pub media_url: String,
}

impl DisasterAttachmentPayload {
    pub fn fixed_media_url(self) -> Self {
        let media_url = CommonUtility::generate_media_url(self.media_url);
        Self { media_url }
    }
}

impl DisasterAttachmentPayload {
    pub fn into_insert(self, disaster_report_id: Uuid) -> DisasterReportsAttachmentModelPayload {
        DisasterReportsAttachmentModelPayload {
            disaster_report_id,
            media_url: self.media_url,
        }
    }
}
