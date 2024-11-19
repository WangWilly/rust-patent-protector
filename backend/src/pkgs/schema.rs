// @generated automatically by Diesel CLI.

diesel::table! {
    test_logs (id) {
        id -> Int4,
        log -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
