use uuid::Uuid;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::disaster_report_aids)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DisasterReportImageModel {
    pub id: Uuid,
    pub disaster_report_id: Uuid,
}
