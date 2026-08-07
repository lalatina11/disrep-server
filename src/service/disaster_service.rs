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
    models::{
        disaster_model::{CreateDisasterReport, CreateDisasterReportPayload, DisasterReportsModel},
        form_data::ImageFormData,
    },
    service::supabase_service::SupabaseService,
    utils::CommonUtility,
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

    pub fn insert(record: CreateDisasterReport) -> Result<DisasterReportsModel, ServiceError> {
        let conn = &mut Database::establish_connection();
        use crate::schema::disaster_reports;
        let res: Result<DisasterReportsModel, Error> = diesel::insert_into(disaster_reports::table)
            .values(record)
            .returning(DisasterReportsModel::as_returning())
            .get_result(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => {
                println!("{err}");
                Err(ServiceError {
                    message: "Failed to create a Disaster Report".to_string(),
                    status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                })
            }
        }
    }

    pub async fn parse_multipart(
        mut multipart: Multipart,
    ) -> Result<(CreateDisasterReportPayload, ImageFormData), ServiceError> {
        let mut payload = CreateDisasterReportPayload {
            title: "".to_string(),
            description: Some("".to_string()),
            street: Some("".to_string()),
            city: "".to_string(),
            lat: 0.0,
            lng: 0.0,
            image: "".to_string(),
            image_storage_url: "".to_string(),
        };
        let mut image = ImageFormData {
            name: "".to_string(),
            bytes: None,
            content_type: None,
        };
        while let Some(field) = multipart.next_field().await? {
            let name = field.name().unwrap_or("");

            match name {
                "image" => {
                    let filename = field.file_name().unwrap_or("image.jpg").to_string();
                    let content_type = field.content_type().map(|s| s.to_string());
                    image.name = filename;
                    image.content_type = content_type;
                    let bytes = field.bytes().await?;
                    image.bytes = Some(bytes);
                }
                "title" => {
                    payload.title = field.text().await.unwrap_or("no title".to_string());
                }
                "description" => {
                    let description = field.text().await;
                    payload.description = match description {
                        Ok(desc) => Some(desc),
                        Err(_) => None,
                    };
                }
                "street" => {
                    let street = field.text().await;
                    payload.street = match street {
                        Ok(desc) => Some(desc),
                        Err(_) => None,
                    };
                }
                "city" => {
                    payload.city = field.text().await.unwrap_or("no city".to_string());
                }
                "lat" => {
                    let string_lat = field.text().await.unwrap_or("".to_string());
                    let _lat: f64 = string_lat.parse().unwrap_or(0.0);
                    payload.lat = _lat;
                }
                "lng" => {
                    let string_lng = field.text().await.unwrap_or("".to_string());
                    let _lng: f64 = string_lng.parse().unwrap_or(0.0);
                    payload.lng = _lng;
                }
                _ => {}
            };
        }
        Ok((payload, image))
    }

    pub async fn create(
        user_id: Uuid,
        multipart: Multipart,
    ) -> Result<DisasterReportsModel, ServiceError> {
        let (mut payload, image) = Self::parse_multipart(multipart).await?;
        let storage_res = SupabaseService::upload_image(image).await?;

        payload.image = CommonUtility::generate_image_url(storage_res.key.clone());
        payload.image_storage_url = storage_res.key;

        Self::insert(payload.into_record(user_id))
    }
}
