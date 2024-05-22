// @generated automatically by Diesel CLI.

diesel::table! {
    conversations (uuid) {
        uuid -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        receiving_user_uuid -> Uuid,
        sending_user_uuid -> Uuid,
    }
}

diesel::table! {
    messages (uuid) {
        uuid -> Uuid,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        read_at -> Nullable<Timestamp>,
        user_uuid -> Uuid,
        conversation_uuid -> Uuid,
    }
}

diesel::joinable!(messages -> conversations (conversation_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    conversations,
    messages,
);
