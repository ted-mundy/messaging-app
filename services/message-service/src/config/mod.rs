use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
  pub server_address: String,
  pub pg: deadpool_postgres::Config,
}
