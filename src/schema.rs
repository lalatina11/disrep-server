// @generated automatically by Diesel CLI.

diesel::table! {
    disaster_report_images (id) {
        id -> Uuid,
        disaster_report_id -> Uuid,
        url -> Text,
    }
}

diesel::table! {
    disaster_reports (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
        street -> Nullable<Text>,
        #[max_length = 100]
        city -> Varchar,
        lat -> Float8,
        lng -> Float8,
        is_anon -> Nullable<Bool>,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        #[max_length = 255]
        display_name -> Varchar,
        role -> Text,
        avatar -> Nullable<Text>,
        avatar_storage_url -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(disaster_report_images -> disaster_reports (disaster_report_id));
diesel::joinable!(disaster_reports -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(disaster_report_images, disaster_reports, users,);
