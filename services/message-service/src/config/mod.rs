use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct AppConfig {
  pub server_address: String,
  pub postgres_url: String,
  pub max_connections: u32,
}
