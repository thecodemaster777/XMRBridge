// Copyright (c) 2025 The Market Index LLC
// All rights reserved.

// gateway/src/db/mod.rs
//! Database module for XMRBridge
//! Sets up SQLx connection pool and provides a central access point for DB queries.

use sqlx::{Pool, Postgres};
use crate::config::Config;
use anyhow::Result;

pub type DbPool = Pool<Postgres>;

/// Initialize the database pool using the config
pub async fn init_db(config: &Config) -> Result<DbPool> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await?;

    // Optional: run migrations on startup if using sqlx migrate
    // sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
