use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Conversation {
    uuid: Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    receiving_user_uuid: Uuid,
    sending_user_uuid: Uuid,
}
