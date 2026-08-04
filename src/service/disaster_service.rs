use diesel::{RunQueryDsl, SelectableHelper, query_dsl::methods::SelectDsl, result::Error};
use reqwest::StatusCode;

use crate::{
    config::database_config::Database,
    error::ServiceError,
    models::disaster_model::{CreateDisasterReport, DisasterReportsModel},
};

pub struct DisasterService;

impl DisasterService {
    pub fn get_all() -> Result<Vec<DisasterReportsModel>, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_reports::dsl::*;
        let res: Result<Vec<DisasterReportsModel>, Error> = disaster_reports
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
}
