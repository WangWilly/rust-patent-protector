// @generated automatically by Diesel CLI.

diesel::table! {
    assessments (id) {
        id -> Int4,
        patent_id -> Text,
        company_name -> Text,
        analysis_date -> Timestamp,
        top_infringing_products -> Nullable<Jsonb>,
        overall_risk_assessment -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    test_logs (id) {
        id -> Int4,
        log -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(assessments, test_logs,);
