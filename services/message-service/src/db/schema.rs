#[macro_use] extern crate diesel;

table! {
    messages (uuid) {
        uuid -> Uuid,
        /// text, as can be either encrypted or not
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        conversation_id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        read_at -> Nullable<Timestamp>,
    }
}
