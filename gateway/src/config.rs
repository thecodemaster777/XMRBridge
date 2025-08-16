// Copyright (c) 2025 The Market Index LLC
// All rights reserved.

// TODO: Load config from env/file using config + clap

// gateway/src/config.rs

use serde::Deserialize;
use std::env;
use std::path::PathBuf;
use clap::Parser;

/// Application configuration loaded from env, config file, and CLI args.
#[derive(Debug, Deserialize, Parser)]
#[clap(name = "XMRBridge Backend", version = "0.1.0")]
pub struct Config {
    /// Path to the config file (optional, defaults to ./config.toml)
    #[clap(short, long)]
    pub config_file: Option<PathBuf>,

    /// Database URL (Postgres or SQLite)
    #[serde(default = "default_db_url")]
    pub database_url: String,

    /// Monero wallet RPC endpoint (http://127.0.0.1:18082)
    #[serde(default = "default_monero_rpc")]
    pub monero_rpc_url: String,

    /// RPC username
    #[serde(default)]
    pub monero_rpc_user: String,

    /// RPC password
    #[serde(default)]
    pub monero_rpc_password: String,

    /// API server host
    #[serde(default = "default_server_host")]
    pub server_host: String,

    /// API server port
    #[serde(default = "default_server_port")]
    pub server_port: u16,
}

fn default_db_url() -> String {
    "postgres://user:password@localhost/xmrbridge".to_string()
}

fn default_monero_rpc() -> String {
    "http://127.0.0.1:18082".to_string()
}

fn default_server_host() -> String {
    "127.0.0.1".to_string()
}

fn default_server_port() -> u16 {
    8080
}

impl Config {
    /// Load configuration from CLI args, environment variables, and optional config file.
    pub fn load() -> anyhow::Result<Self> {
        // First, parse CLI args
        let mut cfg = Config::parse();

        // If a config file is provided or default exists, merge it
        let config_path = cfg
            .config_file
            .clone()
            .unwrap_or_else(|| PathBuf::from("config.toml"));

        if config_path.exists() {
            let file_cfg = config::Config::builder()
                .add_source(config::File::from(config_path))
                .build()?;
            let file_cfg: Config = file_cfg.try_deserialize()?;
            // Merge file config into CLI/env config (CLI takes precedence)
            cfg = Config {
                database_url: file_cfg.database_url,
                monero_rpc_url: file_cfg.monero_rpc_url,
                monero_rpc_user: file_cfg.monero_rpc_user,
                monero_rpc_password: file_cfg.monero_rpc_password,
                server_host: file_cfg.server_host,
                server_port: file_cfg.server_port,
                ..cfg
            };
        }

        // Override with environment variables if present (e.g., XMRBRIDGE_DATABASE_URL)
        if let Ok(db_url) = env::var("XMRBRIDGE_DATABASE_URL") {
            cfg.database_url = db_url;
        }
        if let Ok(monero_url) = env::var("XMRBRIDGE_MONERO_RPC_URL") {
            cfg.monero_rpc_url = monero_url;
        }

        Ok(cfg)
    }
}

// TODO: Add methods for validating config (ports, URLs, credentials)

