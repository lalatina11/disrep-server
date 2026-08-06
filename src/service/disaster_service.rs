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

    pub fn create(payload: CreateDisasterReport) -> Result<DisasterReportsModel, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_reports;
        let res: Result<DisasterReportsModel, Error> = diesel::insert_into(disaster_reports::table)
            .values(payload)
            .returning(DisasterReportsModel::as_returning())
            .get_result(conn);
        if let Ok(data) = res {
            return Ok(data);
        }

        if let Err(_) = res {
            return Err(ServiceError {
                message: "Failed to create a Disaster Report".to_string(),
                status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            });
        }

        Err(ServiceError::internal())
    }
    pub async fn upload(
        user_id: Uuid,
        multipart: Multipart,
    ) -> Result<DisasterReportsModel, ServiceError> {
        let _payload = SupabaseService::upload_image(multipart).await;

        if let Err(err) = _payload {
            return Err(err);
        }

        if let Ok(payload) = _payload {
            let insert = DisasterService::create(payload.into_record(user_id));
            if let Ok(result) = insert {
                return Ok(result);
            } else if let Err(err) = insert {
                return Err(err);
            }
        }

        Err(ServiceError::internal())
    }
}
