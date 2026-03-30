use axum::{Extension, extract::FromRequestParts};
use centaurus::{
  config::{BaseConfig, MetricsConfig},
  db::config::DBConfig,
};
use figment::{
  Figment,
  providers::{Env, Serialized},
};
use serde::{Deserialize, Serialize};
use tracing::{instrument, warn};
use url::Url;

#[derive(Deserialize, Serialize, Clone, FromRequestParts)]
#[from_request(via(Extension))]
pub struct Config {
  #[serde(flatten)]
  pub base: BaseConfig,
  #[serde(flatten)]
  pub db: DBConfig,
  #[serde(flatten)]
  pub metrics: MetricsConfig,

  pub db_url: String,
  pub site_url: Url,

  pub auth_pepper: String,
  pub auth_issuer: String,
  pub auth_jwt_expiration: i64,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      base: BaseConfig::default(),
      db: DBConfig::default(),
      db_url: "".to_string(),
      site_url: Url::parse("http://localhost:8000").unwrap(),
      metrics: MetricsConfig {
        metrics_name: "ichwilldich-sep".to_string(),
        ..Default::default()
      },
      auth_pepper: "__ICHWILLDICH_SEP_PEPPER__".to_string(),
      auth_issuer: "ichwilldich_sep_auth".to_string(),
      auth_jwt_expiration: 60 * 60 * 24 * 7, // 7 days
    }
  }
}

impl Config {
  #[instrument]
  pub fn parse() -> Self {
    let config = Figment::new()
      .merge(Serialized::defaults(Self::default()))
      .merge(Env::raw().global());

    let mut config: Self = config.extract().expect("Failed to parse configuration");

    if config.db_url.is_empty() {
      panic!("DB_URL is not set");
    }

    if config.db_url.starts_with("sqlite") {
      if config.db.database_max_connections > 1 {
        config.db.database_max_connections = 1;
        if config.db.database_max_connections != DBConfig::default().database_max_connections {
          warn!(
            "SQLite does not work properly with multiple connections. Setting DATABASE_MAX_CONNECTIONS to 1."
          );
        }
      }

      if config.db.database_min_connections > 1 {
        config.db.database_min_connections = 1;
        if config.db.database_min_connections != DBConfig::default().database_min_connections {
          warn!(
            "SQLite does not work properly with multiple connections. Setting DATABASE_MIN_CONNECTIONS to 1."
          );
        }
      }
    }

    config
  }
}
