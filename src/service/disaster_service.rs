use chrono::Utc;
use diesel::{
    ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, result::Error as DieselError,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::database_config::Database,
    error::ServiceError,
    models::{
        disaster_aid_model::DisasterAidWithAllRelations,
        disaster_model::{CreateDisasterReportWithImage, DisasterReportsModel, DisasterStatus},
        disaster_report_attachment_model::{
            DisasterAttachmentPayload, DisasterReportAttachmentModel,
            DisasterReportsAttachmentModelPayload,
        },
        user_model::UserModel,
    },
    service::{
        disaster_attachment_service::DisasterAttachmentService,
        disaster_report_aid_service::DiassterReportAidService, user_service::UserService,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterWithAllRelations {
    pub disaster: DisasterReportsModel,
    pub attachments: Vec<DisasterAttachmentPayload>,
    pub author: UserModel,
    pub aids: Vec<DisasterAidWithAllRelations>,
}

pub struct DisasterService;

impl DisasterService {
    pub async fn get_all() -> Result<Vec<DisasterWithAllRelations>, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::{disaster_report_attachments, disaster_reports, users};
        let report_with_users_res: Result<Vec<(DisasterReportsModel, UserModel)>, DieselError> =
            disaster_reports::table
                .filter(disaster_reports::status.ne("pending"))
                .inner_join(users::table)
                .select((DisasterReportsModel::as_select(), UserModel::as_select()))
                .load::<(DisasterReportsModel, UserModel)>(conn);

        if let Ok(reports_with_users) = report_with_users_res {
            let report_ids: Vec<Uuid> = reports_with_users
                .iter()
                .map(|(report, _)| report.id)
                .collect();
            let all_attachments: Result<Vec<DisasterReportAttachmentModel>, DieselError> =
                disaster_report_attachments::table
                    .filter(disaster_report_attachments::disaster_report_id.eq_any(&report_ids))
                    .select(DisasterReportAttachmentModel::as_select())
                    .load::<DisasterReportAttachmentModel>(conn);
            if let Ok(attachments) = all_attachments {
                let aid_res =
                    DiassterReportAidService::get_all(report_ids.iter().cloned().collect()).await;
                if let Ok(aid_data) = aid_res {
                    let result = reports_with_users
                        .into_iter()
                        .map(|(disaster, author)| {
                            let attachments = attachments
                                .iter()
                                .filter(|img| &img.disaster_report_id == &disaster.id)
                                .map(|img| {
                                    let disaster_image = DisasterAttachmentPayload {
                                        media_url: img.media_url.clone(),
                                    };
                                    disaster_image.fixed_media_url()
                                })
                                .collect();
                            let aids = aid_data
                                .iter()
                                .filter(|aid| &aid.disaster_report_id == &disaster.id)
                                .cloned()
                                .collect();
                            DisasterWithAllRelations {
                                disaster,
                                attachments,
                                author,
                                aids,
                            }
                        })
                        .collect();
                    return Ok(result);
                }
            }
        }
        Err(ServiceError::internal())
    }

    pub async fn create(
        user_id: Uuid,
        mut payload: CreateDisasterReportWithImage,
    ) -> Result<DisasterReportsModel, ServiceError> {
        payload.validate()?;

        let user = UserService::get_user_by_id(user_id).await?;

        if user.is_authorize_as_admins() {
            payload.status = Some(DisasterStatus::New.to_string())
        }

        use crate::schema::disaster_reports;

        let conn = &mut Database::establish_connection();

        let disaster_record = payload.to_record(user_id);

        let insert_disaster_res: Result<DisasterReportsModel, DieselError> =
            diesel::insert_into(disaster_reports::table)
                .values(disaster_record)
                .returning(DisasterReportsModel::as_returning())
                .get_result(conn);

        if let Err(_) = &insert_disaster_res {
            return Err(ServiceError::internal());
        }

        if let Ok(disaster) = insert_disaster_res {
            let disaster_attachment_payload: Vec<DisasterReportsAttachmentModelPayload> = payload
                .attachment
                .into_iter()
                .map(|attachment| attachment.into_insert(disaster.id))
                .collect();
            let res = DisasterAttachmentService::insert(disaster_attachment_payload).await;
            if let Err(_) = res {
                return Err(ServiceError::internal());
            }
            return Ok(disaster);
        }

        Err(ServiceError::internal())
    }

    pub async fn get_by_id(disaster_id: Uuid) -> Result<DisasterWithAllRelations, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::{disaster_report_attachments, disaster_reports, users};
        let res: Result<(DisasterReportsModel, UserModel), DieselError> = disaster_reports::table
            .find(disaster_id)
            .inner_join(users::table)
            .select((DisasterReportsModel::as_select(), UserModel::as_select()))
            .first(conn);

        if let Err(_) = res {
            return Err(ServiceError::not_found(Some(
                "Disaster not found".to_string(),
            )));
        }

        if let Ok((disaster, author)) = res {
            let attachment_res: Result<Vec<DisasterReportAttachmentModel>, DieselError> =
                disaster_report_attachments::table
                    .filter(
                        disaster_report_attachments::columns::disaster_report_id.eq(disaster.id),
                    )
                    .select(DisasterReportAttachmentModel::as_select())
                    .load(conn);
            if let Ok(attachments) = attachment_res {
                let diaster_aid_res = DiassterReportAidService::get_all(vec![disaster_id]).await;
                let attachments = attachments
                    .into_iter()
                    .map(|attch| {
                        let attachment = DisasterAttachmentPayload {
                            media_url: attch.media_url,
                        };
                        attachment.fixed_media_url()
                    })
                    .collect();
                if let Ok(aids) = diaster_aid_res {
                    return Ok(DisasterWithAllRelations {
                        disaster,
                        author,
                        attachments,
                        aids: aids,
                    });
                }
            }
        }

        Err(ServiceError::internal())
    }

    pub async fn approve(_id: Uuid) -> Result<DisasterReportsModel, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_reports::dsl::*;
        let _disaster = Self::get_by_id(_id).await?;

        let res: Result<DisasterReportsModel, DieselError> =
            diesel::update(disaster_reports.find(_disaster.disaster.id))
                .set((status.eq("new".to_string()), updated_at.eq(Utc::now())))
                .returning(DisasterReportsModel::as_returning())
                .get_result(conn);

        match res {
            Err(_) => Err(ServiceError::internal()),
            Ok(data) => Ok(data),
        }
    }

    pub async fn check_existing(disaster_id: Uuid) -> bool {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_reports;
        let res: Result<DisasterReportsModel, DieselError> = disaster_reports::table
            .find(disaster_id)
            .select(DisasterReportsModel::as_select())
            .first(conn);

        if let Err(_) = res {
            return false;
        }

        true
    }

    pub async fn update_status(
        disaster_id: Uuid,
        status: DisasterStatus,
    ) -> Result<DisasterWithAllRelations, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_reports;
        let disaster_update_res: Result<DisasterReportsModel, DieselError> =
            diesel::update(disaster_reports::dsl::disaster_reports.find(disaster_id))
                .set(disaster_reports::dsl::status.eq(status.to_string()))
                .returning(DisasterReportsModel::as_returning())
                .get_result(conn);

        if let Ok(update_result) = disaster_update_res {
            let disaster_res = Self::get_by_id(update_result.id).await;
            if let Ok(disaster) = disaster_res {
                return Ok(disaster);
            }
        }

        Err(ServiceError::internal())
    }
}
