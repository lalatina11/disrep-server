use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::disaster_report_images)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct DisasterReportsImageModel {
    pub id: Uuid,
    pub disaster_report_id: Uuid,
    pub url: String,
}
