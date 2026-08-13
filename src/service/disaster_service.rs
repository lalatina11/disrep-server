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
        disaster_model::{
            CreateDisasterReportWithImage, DisasterReportImageModel, DisasterReportsModel,
            DisasterStatus,
        },
        user_model::UserModel,
    },
    service::{disaster_image_service::DisasterImageService, user_service::UserService},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterWithAllRelations {
    pub disaster: DisasterReportsModel,
    pub images: Vec<DisasterReportImageModel>,
    pub author: UserModel,
}

pub struct DisasterService;

impl DisasterService {
    pub fn get_all() -> Result<Vec<DisasterWithAllRelations>, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::{disaster_report_images, disaster_reports, users};
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
            let all_images: Result<Vec<DisasterReportImageModel>, DieselError> =
                disaster_report_images::table
                    .filter(disaster_report_images::disaster_report_id.eq_any(&report_ids))
                    .select(DisasterReportImageModel::as_select())
                    .load::<DisasterReportImageModel>(conn);
            if let Ok(images) = all_images {
                let result = reports_with_users
                    .into_iter()
                    .map(|(disaster, author)| {
                        let images = images
                            .iter()
                            .filter(|img| &img.disaster_report_id == &disaster.id)
                            .cloned()
                            .collect();

                        DisasterWithAllRelations {
                            disaster,
                            author,
                            images,
                        }
                    })
                    .collect();
                return Ok(result);
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

        if user.is_authorize_as_admin() {
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
            for attachment in payload.attachment {
                attachment.validate()?;
                let payload = attachment.into_insert(disaster.id);

                let res = DisasterImageService::insert(payload).await;
                if let Err(_) = res {
                    return Err(ServiceError::internal());
                }
            }
            return Ok(disaster);
        }

        Err(ServiceError::internal())
    }

    pub async fn get_by_id(disaster_id: Uuid) -> Result<DisasterWithAllRelations, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::{disaster_report_images, disaster_reports, users};
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
            let images_res: Result<Vec<DisasterReportImageModel>, DieselError> =
                disaster_report_images::table
                    .filter(disaster_report_images::columns::disaster_report_id.eq(disaster.id))
                    .select(DisasterReportImageModel::as_select())
                    .load(conn);
            if let Ok(images) = images_res {
                return Ok(DisasterWithAllRelations {
                    disaster,
                    author,
                    images,
                });
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
}
