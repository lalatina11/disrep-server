use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::utils::CommonUtility;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=crate::schema::disaster_report_aid_attachments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DisasterAidAttachmentModel {
    pub id: Uuid,
    pub disaster_report_aid_id: Uuid,
    pub media_url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name=crate::schema::disaster_report_aid_attachments)]
pub struct DisasterAidAttachmentPayloadWidhDisasterAidId {
    pub disaster_report_aid_id: Uuid,
    pub media_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DisasterAidAttachmentPayload {
    #[validate(length(min = 1, message = "Invalid attachment URL"))]
    pub media_url: String,
}

impl DisasterAidAttachmentPayload {
    pub fn to_records(
        &self,
        disaster_report_aid_id: Uuid,
    ) -> DisasterAidAttachmentPayloadWidhDisasterAidId {
        DisasterAidAttachmentPayloadWidhDisasterAidId {
            disaster_report_aid_id,
            media_url: self.media_url.clone(),
        }
    }

    pub fn fixed_media_url(self) -> Self {
        let media_url = CommonUtility::generate_media_url(self.media_url);
        Self { media_url }
    }
}
