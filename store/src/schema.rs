// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "website_status"))]
    pub struct WebsiteStatus;
}

diesel::table! {
    region (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        email -> Text,
        user_name -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteStatus;

    website_tick (id) {
        id -> Text,
        response_time_ms -> Int4,
        status -> WebsiteStatus,
        website_id -> Text,
        region_id -> Text,
    }
}

diesel::table! {
    websites (id) {
        id -> Text,
        url -> Text,
        time_added -> Timestamp,
        user_id -> Text,
    }
}

diesel::joinable!(website_tick -> region (region_id));
diesel::joinable!(website_tick -> websites (website_id));
diesel::joinable!(websites -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(region, users, website_tick, websites,);
