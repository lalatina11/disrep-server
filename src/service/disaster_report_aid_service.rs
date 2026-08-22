use diesel::{
    ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl, result::Error as DieselError,
};
use uuid::Uuid;

use crate::{
    config::database_config::Database,
    error::ServiceError,
    models::{
        disaster_aid_attachment_model::DisasterAidAttachmentModel,
        disaster_aid_items_model::DisasterAidItemModel,
        disaster_aid_model::{DisasterAidModel, DisasterAidWithAllRelations},
    },
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
}
