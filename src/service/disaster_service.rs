use chrono::Utc;
use diesel::{
    ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, result::Error as DieselError,
};
use uuid::Uuid;

use crate::{
    config::database_config::Database,
    error::ServiceError,
    models::disaster_model::{CreateDisasterReportWithImage, DisasterReportsModel, DisasterStatus},
    service::{disaster_image_service::DisasterImageService, user_service::UserService},
};

pub struct DisasterService;

impl DisasterService {
    pub fn get_all() -> Result<Vec<DisasterReportsModel>, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_reports::dsl::*;
        let res: Result<Vec<DisasterReportsModel>, DieselError> = disaster_reports
            .filter(status.ne("pending"))
            .select(DisasterReportsModel::as_select())
            .load(conn);
        if let Ok(data) = res {
            return Ok(data);
        }
        Err(ServiceError::internal())
    }

    pub async fn create(
        user_id: Uuid,
        mut payload: CreateDisasterReportWithImage,
    ) -> Result<DisasterReportsModel, ServiceError> {
        if payload.attachment.len() < 1 {
            return Err(ServiceError::unprocessable(Some(
                "Please insert an image or video".to_string(),
            )));
        }

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
            for img in payload.attachment {
                let payload = img.into_insert(disaster.id);
                let res = DisasterImageService::insert(payload).await;
                if let Err(_) = res {
                    return Err(ServiceError::internal());
                }
            }
            return Ok(disaster);
        }

        Err(ServiceError::internal())
    }

    pub async fn get_by_id(disaster_id: Uuid) -> Result<DisasterReportsModel, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_reports::dsl::*;
        let res: Result<DisasterReportsModel, DieselError> = disaster_reports
            .find(disaster_id)
            .select(DisasterReportsModel::as_select())
            .first(conn);

        match res {
            Err(_) => Err(ServiceError::not_found(Some(
                "Disaster Report was not found".to_string(),
            ))),
            Ok(data) => Ok(data),
        }
    }

    pub async fn approve(_id: Uuid) -> Result<DisasterReportsModel, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_reports::dsl::*;
        let _disaster = Self::get_by_id(_id).await?;

        let res: Result<DisasterReportsModel, DieselError> =
            diesel::update(disaster_reports.find(_disaster.id))
                .set((status.eq("new".to_string()), updated_at.eq(Utc::now())))
                .returning(DisasterReportsModel::as_returning())
                .get_result(conn);

        match res {
            Err(_) => Err(ServiceError::internal()),
            Ok(data) => Ok(data),
        }
    }
}
