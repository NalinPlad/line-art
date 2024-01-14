// @generated automatically by Diesel CLI.

diesel::table! {
    drawings (id) {
        id -> Int4,
        lines -> Text,
        artist -> Text,
        created_at -> Timestamp,
    }
}
