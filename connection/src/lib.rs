use anyhow::Result;
use std::str::FromStr;

use deadpool_postgres::{Manager, ManagerConfig, Pool};
use tokio_postgres::NoTls;

pub async fn create_pool(db_url: &str) -> Result<Pool> {
    let pg_config = tokio_postgres::Config::from_str(db_url)?;
    let mgr = Manager::from_config(pg_config, NoTls, ManagerConfig::default());
    Pool::builder(mgr).max_size(30).build().map_err(Into::into)
}
