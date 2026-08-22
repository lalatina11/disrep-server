use std::borrow::Cow;

use diesel::prelude::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::disaster_report_aid_items)]
pub struct DisasterAidItemModel {
    pub id: Uuid,
    pub disaster_report_aid_id: Uuid,
    pub item_name: String,
    pub item_price: f64,
    pub quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DisasterAidItemPayload {
    #[validate(length(min = 3, max = 255, message = "Required 3-255 characters"))]
    pub item_name: String,
    #[validate(custom(function = "validate_price"))]
    pub item_price: f64,
    #[validate(range(
        min = 1,
        max = 1_000_000,
        message = "Quantity must between 1-1.000.000"
    ))]
    pub quantity: i64,
}

impl DisasterAidItemPayload {
    pub fn to_records(
        &self,
        disaster_report_aid_id: Uuid,
    ) -> DisasterAidItemPayloadWithDisasterAidId {
        DisasterAidItemPayloadWithDisasterAidId {
            disaster_report_aid_id,
            item_name: self.item_name.clone(),
            item_price: self.item_price,
            quantity: self.quantity,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Insertable)]
#[diesel(table_name = crate::schema::disaster_report_aid_items)]
pub struct DisasterAidItemPayloadWithDisasterAidId {
    pub disaster_report_aid_id: Uuid,
    #[validate(length(min = 3, max = 255, message = "Required 3-255 characters"))]
    pub item_name: String,
    #[validate(custom(function = "validate_price"))]
    pub item_price: f64,
    #[validate(range(
        min = 1,
        max = 1_000_000,
        message = "Quantity must between 1-1.000.000"
    ))]
    pub quantity: i64,
}

fn validate_price(price: f64) -> Result<(), ValidationError> {
    if price < 0.0 || price.is_nan() || price.is_infinite() {
        let mut err = ValidationError::new("invalid_price");
        err.message = Some(Cow::from("Invalid Price"));
        return Err(err);
    }
    if price < 1000.0 {
        let mut err = ValidationError::new("invalid_price");
        err.message = Some(Cow::from("Minimum Rp.1000"));
        return Err(err);
    }
    Ok(())
}
