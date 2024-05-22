use diesel::{self, query_dsl::methods::LimitDsl, r2d2::{ConnectionManager, PooledConnection}, QueryResult, RunQueryDsl};

use crate::models::conversation::Conversation;

pub async fn get_conversations(
  mut db_conn: PooledConnection<ConnectionManager<diesel::PgConnection>>,
) -> QueryResult<Conversation> {
  use crate::db::schema::conversations::dsl::*;

  conversations
    .limit(10)
    .get_result::<Conversation>(&mut db_conn)
}
