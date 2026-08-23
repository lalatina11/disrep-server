use diesel::{
    ExpressionMethods, RunQueryDsl, SelectableHelper, query_dsl::methods::FilterDsl,
    result::Error as DieselError,
};

use uuid::Uuid;
use validator::Validate;

use crate::{
    config::database_config::Database,
    error::ServiceError,
    models::{
        disaster_aid_attachment_model::{
            DisasterAidAttachmentModel, DisasterAidAttachmentPayloadWidhDisasterAidId,
        },
        disaster_aid_items_model::{DisasterAidItemModel, DisasterAidItemPayloadWithDisasterAidId},
        disaster_aid_model::{CreateDisasterAid, DisasterAidModel, DisasterAidWithAllRelations},
        disaster_model::{DisasterStatus, UpdateDisasterStatusPayload},
    },
    service::disaster_service::{DisasterService, DisasterWithAllRelations},
};

pub struct DiassterReportAidService;

impl DiassterReportAidService {
    pub async fn get_all(
        disaster_report_ids: Vec<Uuid>,
    ) -> Result<Vec<DisasterAidWithAllRelations>, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::{
            disaster_report_aid_attachments, disaster_report_aid_items, disaster_report_aids,
        };
        let disaster_aid_res: Result<Vec<DisasterAidModel>, DieselError> =
            disaster_report_aids::table
                .filter(disaster_report_aids::disaster_report_id.eq_any(&disaster_report_ids))
                .load(conn);
        if let Ok(disaster_aid_data) = disaster_aid_res {
            let disaster_aid_ids: Vec<Uuid> = disaster_aid_data
                .iter()
                .map(|disaster_aid| disaster_aid.id)
                .collect();
            let disaster_aid_attachment_res: Result<Vec<DisasterAidAttachmentModel>, DieselError> =
                disaster_report_aid_attachments::table
                    .filter(
                        disaster_report_aid_attachments::disaster_report_aid_id
                            .eq_any(&disaster_aid_ids),
                    )
                    .load(conn);
            if let Ok(disaster_aid_attachment_data) = disaster_aid_attachment_res {
                let disaster_aid_items_res: Result<Vec<DisasterAidItemModel>, DieselError> =
                    disaster_report_aid_items::table
                        .filter(
                            disaster_report_aid_items::disaster_report_aid_id
                                .eq_any(&disaster_aid_ids),
                        )
                        .load(conn);
                if let Ok(disaster_aid_items_data) = disaster_aid_items_res {
                    let aids: Vec<DisasterAidWithAllRelations> = disaster_aid_data
                        .iter()
                        .map(|aid| {
                            let attachments: Vec<DisasterAidAttachmentModel> =
                                disaster_aid_attachment_data
                                    .iter()
                                    .filter(|attachment| {
                                        &attachment.disaster_report_aid_id == &aid.id
                                    })
                                    .cloned()
                                    .collect();
                            let items: Vec<DisasterAidItemModel> = disaster_aid_items_data
                                .iter()
                                .filter(|item| &item.disaster_report_aid_id == &aid.id)
                                .cloned()
                                .collect();
                            DisasterAidWithAllRelations {
                                id: aid.id,
                                disaster_report_id: aid.disaster_report_id,
                                attachments,
                                items,
                            }
                        })
                        .collect();
                    return Ok(aids);
                }
            }
        }
        Err(ServiceError::internal())
    }

    pub async fn create(
        payload: CreateDisasterAid,
    ) -> Result<DisasterWithAllRelations, ServiceError> {
        payload.validate()?;

        let disaster_id = payload.disaster_id;

        let is_exist = DisasterService::check_existing(disaster_id).await;

        if !is_exist {
            return Err(ServiceError::unprocessable(Some(
                "Disaster Report Not Found".to_string(),
            )));
        }

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
            let update_payload = UpdateDisasterStatusPayload {
                status: DisasterStatus::AidDispatched.to_string(),
            };
            let disaster_res = DisasterService::update_status(disaster_id, update_payload).await;
            if let Ok(disaster) = disaster_res {
                return Ok(disaster);
            }
        }
        Err(ServiceError::internal())
    }
}
