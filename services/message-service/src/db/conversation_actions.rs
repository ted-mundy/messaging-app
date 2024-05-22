use std::borrow::{Borrow, BorrowMut};

use diesel::{self, r2d2::{ConnectionManager, PooledConnection}, ExpressionMethods, QueryDsl};
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

pub async fn get_conversations(
  mut config: GetConversationConfig,
) -> PaginatedResponse<Conversation> {
  use crate::db::schema::conversations::dsl::*;

  let mut query = conversations
    .into_boxed();

  query = query.filter(receiving_user_uuid.eq(config.filter.user_uuid));
  query = query.or_filter(sending_user_uuid.eq(config.filter.user_uuid));

  query = query.order(created_at.desc());

  let paginated = query.paginate(config.page).per_page(config.per_page);

  let (data, total, total_pages) = paginated.load_and_count_pages(&mut config.db_pool).unwrap();

  PaginatedResponse {
    data,
    total: total_pages,
    page: config.page,
    per_page: config.per_page,
    total_items: total,
  }
}
