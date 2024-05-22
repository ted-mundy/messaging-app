use diesel::{query_builder::AsQuery, query_dsl::methods::LoadQuery, r2d2::{self, ConnectionManager}, IntoSql};

pub mod message_actions;
pub mod conversation_actions;
pub mod schema;
pub mod pagination;

pub fn create_pool(config: crate::config::AppConfig) -> r2d2::Pool<ConnectionManager<diesel::PgConnection>> {
  let manager = ConnectionManager::<diesel::PgConnection>::new(config.postgres_url.to_owned());
  let pool = r2d2::Pool::builder()
    .max_size(config.max_connections)
    .build(manager)
    .expect("Failed to create pool");

  return pool;
}
