use crate::{
    config::database_config::Database,
    error::ServiceError,
    models::disaster_report_attachment_model::{
        DisasterReportAttachmentModel, DisasterReportsAttachmentModelPayload,
    },
};
use diesel::{RunQueryDsl, SelectableHelper, result::Error as DieselError};

pub struct DisasterImageService;

impl DisasterImageService {
    pub async fn insert(
        payload: DisasterReportsAttachmentModelPayload,
    ) -> Result<DisasterReportAttachmentModel, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_report_images;

        let res: Result<DisasterReportAttachmentModel, DieselError> =
            diesel::insert_into(disaster_report_images::table)
                .values(payload)
                .returning(DisasterReportAttachmentModel::as_returning())
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
