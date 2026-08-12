use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::disaster_report_images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DisasterReportsImageModel {
    pub id: Uuid,
    pub disaster_report_id: Uuid,
    pub url: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = crate::schema::disaster_report_images)]
pub struct DisasterReportsImageModelPayload {
    pub disaster_report_id: Uuid,
    pub url: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct DisasterImagePayload {
    #[validate(length(min = 1, message = "Invalid attachment URL"))]
    pub url: String,
}

impl DisasterImagePayload {
    pub fn into_insert(self, disaster_report_id: Uuid) -> DisasterReportsImageModelPayload {
        DisasterReportsImageModelPayload {
            disaster_report_id,
            url: self.url,
        }
    }
}
