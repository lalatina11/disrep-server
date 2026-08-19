use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::disaster_aid_attachment_model::DisasterAidAttachmentPayload;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::disaster_report_aids)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DisasterAidModel {
    pub id: Uuid,
    pub disaster_report_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDisasterAid {
    pub disaster_id: Uuid,
    #[validate(length(min = 1, message = "Please insert an image or video"), nested)]
    pub attachments: Vec<DisasterAidAttachmentPayload>,
}
