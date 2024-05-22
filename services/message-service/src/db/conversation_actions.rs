use diesel::{self, insert_into, prelude::Insertable, r2d2::{ConnectionManager, PooledConnection}, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::pagination::{Paginate, PaginatedResponse}, models::conversation::Conversation};

use super::schema::conversations;

#[derive(Deserialize)]
pub struct ConversationFilter {
  pub user_uuid: Uuid,
}

#[derive(Serialize)]
pub struct ListConversationResponse {
  pub data: Vec<Conversation>,
  pub total: i64,
  pub page: i64,
  pub per_page: i64,
}

pub struct GetConversationConfig {
  pub db_pool: PooledConnection<ConnectionManager<diesel::PgConnection>>,
  pub filter: ConversationFilter,
  pub page: i64,
  pub per_page: i64,
}

pub async fn get_conversations_from_db(
  mut config: GetConversationConfig,
) -> PaginatedResponse<Conversation> {
  use crate::db::schema::conversations::dsl::*;

  let (data, total, total_pages) = conversations
    .filter(receiving_user_uuid.eq(config.filter.user_uuid))
    .or_filter(sending_user_uuid.eq(config.filter.user_uuid))
    .order(created_at.desc())
    .paginate(config.page)
    .per_page(config.per_page)
    .load_and_count_pages(&mut config.db_pool)
    .unwrap();

  PaginatedResponse {
    data,
    total: total_pages,
    page: config.page,
    per_page: config.per_page,
    total_items: total,
  }
}

pub struct CreateConversationConfig {
  pub db_pool: PooledConnection<ConnectionManager<diesel::PgConnection>>,
  pub input: CreateConversationInput,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = conversations)]
pub struct CreateConversationInput {
  pub sending_user_uuid: Uuid,
  pub receiving_user_uuid: Uuid,
}

pub async fn create_conversation(
  mut config: CreateConversationConfig,
) -> Result<Conversation, diesel::result::Error> {
  use crate::db::schema::conversations::dsl::*;

  let result = insert_into(conversations)
    .values(&config.input)
    .get_result(&mut config.db_pool);

  result
}
