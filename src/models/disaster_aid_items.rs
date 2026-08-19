use diesel::prelude::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::disaster_report_aid_items)]
pub struct DisasterAidItems {
    id: Uuid,
    disaster_report_aid_id: Uuid,
    item_name: String,
    item_price: f64,
    quantity: i64,
}
