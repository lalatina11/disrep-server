use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=crate::schema::disaster_report_aid_attachments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DisasterAidAttachmentModel {
    id: Uuid,
    disaster_report_aid_id: Uuid,
    image_url: String,
}
