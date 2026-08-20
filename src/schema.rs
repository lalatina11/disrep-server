// @generated automatically by Diesel CLI.

diesel::table! {
    disaster_report_aid_attachments (id) {
        id -> Uuid,
        disaster_report_aid_id -> Uuid,
        media_url -> Text,
    }
}

diesel::table! {
    disaster_report_aid_items (id) {
        id -> Uuid,
        disaster_report_aid_id -> Uuid,
        #[max_length = 128]
        item_name -> Varchar,
        item_price -> Float8,
        quantity -> Int8,
    }
}

diesel::table! {
    disaster_report_aids (id) {
        id -> Uuid,
        disaster_report_id -> Uuid,
    }
}

diesel::table! {
    disaster_report_attachments (id) {
        id -> Uuid,
        disaster_report_id -> Uuid,
        media_url -> Text,
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

diesel::joinable!(disaster_report_aid_attachments -> disaster_report_aids (disaster_report_aid_id));
diesel::joinable!(disaster_report_aid_items -> disaster_report_aids (disaster_report_aid_id));
diesel::joinable!(disaster_report_aids -> disaster_reports (disaster_report_id));
diesel::joinable!(disaster_report_attachments -> disaster_reports (disaster_report_id));
diesel::joinable!(disaster_reports -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    disaster_report_aid_attachments,
    disaster_report_aid_items,
    disaster_report_aids,
    disaster_report_attachments,
    disaster_reports,
    users,
);
