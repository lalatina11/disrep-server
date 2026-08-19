use crate::{
    config::database_config::Database,
    error::ServiceError,
    models::disaster_report_image_model::{
        DisasterReportImageModel, DisasterReportsImageModelPayload,
    },
};
use diesel::{RunQueryDsl, SelectableHelper, result::Error as DieselError};

pub struct DisasterImageService;

impl DisasterImageService {
    pub async fn insert(
        payload: DisasterReportsImageModelPayload,
    ) -> Result<DisasterReportImageModel, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_report_images;

        let res: Result<DisasterReportImageModel, DieselError> =
            diesel::insert_into(disaster_report_images::table)
                .values(payload)
                .returning(DisasterReportImageModel::as_returning())
                .get_result(conn);

        if let Err(err) = &res {
            println!("{}", err);
        }

        if let Ok(result) = res {
            return Ok(result);
        }

        Err(ServiceError::internal())
    }
}
