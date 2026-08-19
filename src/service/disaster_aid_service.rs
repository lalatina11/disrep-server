use crate::{
    config::database_config::Database,
    error::ServiceError,
    models::{
        disaster_aid_attachment_model::DisasterAidAttachmentPayloadWidhDisasterAidId,
        disaster_aid_items::DisasterAidItemPayloadWithDisasterAidId,
        disaster_aid_model::{CreateDisasterAid, DisasterAidModel},
    },
};
use diesel::{RunQueryDsl, SelectableHelper, result::Error as DieselError};
use validator::Validate;
pub struct DisasterAidService;

impl DisasterAidService {
    pub async fn create(payload: CreateDisasterAid) -> Result<(), ServiceError> {
        payload.validate()?;
        let conn = &mut Database::establish_connection();
        use crate::schema::{
            disaster_report_aid_attachments, disaster_report_aid_items, disaster_report_aids,
        };
        let records = payload.to_records();
        let insert_disaster_aid: Result<DisasterAidModel, DieselError> =
            diesel::insert_into(disaster_report_aids::table)
                .values(records)
                .returning(DisasterAidModel::as_returning())
                .get_result(conn);
        if let Ok(data) = insert_disaster_aid {
            let disaster_aid_item_payload: Vec<DisasterAidItemPayloadWithDisasterAidId> = payload
                .items
                .into_iter()
                .map(|item| item.to_records(data.id))
                .collect();
            let insert_disaster_aid_item: Result<_, DieselError> =
                diesel::insert_into(disaster_report_aid_items::table)
                    .values(disaster_aid_item_payload)
                    .execute(conn);
            if let Err(_) = insert_disaster_aid_item {
                return Err(ServiceError::unprocessable(Some(
                    "Failed to insert disaster aid items".to_string(),
                )));
            }
            let disaster_aid_attachment_payload: Vec<
                DisasterAidAttachmentPayloadWidhDisasterAidId,
            > = payload
                .attachments
                .into_iter()
                .map(|item| item.to_records(data.id))
                .collect();
            let insert_disaster_aid_attachment: Result<_, DieselError> =
                diesel::insert_into(disaster_report_aid_attachments::table)
                    .values(disaster_aid_attachment_payload)
                    .execute(conn);
            if let Err(_) = insert_disaster_aid_attachment {
                return Err(ServiceError::unprocessable(Some(
                    "Failed to insert disaster aid attachments".to_string(),
                )));
            }
            return Ok(());
        }
        Err(ServiceError::internal())
    }
}
