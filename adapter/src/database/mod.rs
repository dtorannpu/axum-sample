pub(crate) mod model;

use shaku::{Component, Interface};
use shared::config::DatabaseConfig;
use shared::error::{AppError, AppResult};
use sqlx::PgPool;
use sqlx::postgres::PgConnectOptions;

fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

#[derive(Clone, Component)]
#[shaku(interface = DbPool)]
pub struct ConnectionPool {
    pool: PgPool,
}

impl ConnectionPool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn inner_ref(&self) -> &PgPool {
        &self.pool
    }

    pub async fn begin(&self) -> AppResult<sqlx::Transaction<'_, sqlx::Postgres>> {
        self.pool.begin().await.map_err(AppError::TransactionError)
    }
}

pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool::new(PgPool::connect_lazy_with(make_pg_connect_options(cfg)))
}

pub trait DbPool: Interface {
    fn get(&self) -> &ConnectionPool;
}

impl DbPool for ConnectionPool {
    fn get(&self) -> &ConnectionPool {
        self
    }
}
