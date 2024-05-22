use diesel::{self, query_dsl::methods::LimitDsl, r2d2::{ConnectionManager, PooledConnection}, QueryResult, RunQueryDsl};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::conversation::Conversation;

#[derive(Deserialize)]
pub struct ConversationFilter {
  pub user_uuid: Option<Uuid>,
}

pub struct GetConversationConfig {
  pub db_pool: PooledConnection<ConnectionManager<diesel::PgConnection>>,
  pub filter: ConversationFilter
}

pub async fn get_conversations(
  mut config: GetConversationConfig,
) -> QueryResult<Conversation> {
  use crate::db::schema::conversations::dsl::*;

  conversations
    .limit(10)
    .get_result::<Conversation>(&mut config.db_pool)
}
