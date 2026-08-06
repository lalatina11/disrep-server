use axum::extract::Multipart;
use diesel::{
    ExpressionMethods, RunQueryDsl, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
    result::Error,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    config::database_config::Database,
    error::ServiceError,
    models::disaster_model::{CreateDisasterReport, DisasterReportsModel},
    service::supabase_service::SupabaseService,
};

pub struct DisasterService;

impl DisasterService {
    pub fn get_all() -> Result<Vec<DisasterReportsModel>, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_reports::dsl::*;
        let res: Result<Vec<DisasterReportsModel>, Error> = disaster_reports
            .filter(status.ne("pending"))
            .select(DisasterReportsModel::as_select())
            .load(conn);
        if let Ok(data) = res {
            return Ok(data);
        }
        Err(ServiceError::internal())
    }

    pub fn insert(payload: CreateDisasterReport) -> Result<DisasterReportsModel, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_reports;
        let res: Result<DisasterReportsModel, Error> = diesel::insert_into(disaster_reports::table)
            .values(payload)
            .returning(DisasterReportsModel::as_returning())
            .get_result(conn);
        match res {
            Ok(data) => Ok(data),
            Err(_) => Err(ServiceError {
                message: "Failed to create a Disaster Report".to_string(),
                status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            }),
        }
    }

    pub async fn create(
        user_id: Uuid,
        multipart: Multipart,
    ) -> Result<DisasterReportsModel, ServiceError> {
        let _payload = SupabaseService::upload_image(multipart).await;

        match _payload {
            Err(err) => Err(err),
            Ok(payload) => {
                let insert = DisasterService::insert(payload.into_record(user_id));
                if let Ok(result) = insert {
                    return Ok(result);
                } else if let Err(err) = insert {
                    return Err(err);
                }
                Err(ServiceError::internal())
            }
        }
    }
}
