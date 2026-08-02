// @generated automatically by Diesel CLI.

diesel::table! {
    disaster_reports (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
        street -> Nullable<Text>,
        #[max_length = 100]
        city -> Varchar,
        lat -> Numeric,
        lng -> Numeric,
        image -> Numeric,
        image_storage_url -> Numeric,
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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(disaster_reports -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(disaster_reports, users,);
