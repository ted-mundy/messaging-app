// @generated automatically by Diesel CLI.

diesel::table! {
    messages (uuid) {
        uuid -> Uuid,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        conversation_id -> Int4,
        user_id -> Int4,
        read_at -> Nullable<Timestamp>,
    }
}
