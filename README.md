# XMRBridge

**XMRBridge** is a backend service for accepting Monero (XMR) payments in e-commerce platforms.  
It provides unique subaddresses per order, real-time price conversion, payment monitoring, and webhook callbacks for merchants.

---

## Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Setup](#setup)
- [Running the Service](#running-the-service)
- [Database Migrations](#database-migrations)
- [Testing](#testing)
- [License](#license)

---

## Features

- **Subaddress Generator:** Generate unique XMR addresses per order.  
- **Price Engine:** Fetch live fiat/XMR conversion rates (CoinGecko, Kraken).  
- **Payment Watcher:** Listen for incoming payments via Monero wallet RPC.  
- **Callback System:** Send signed webhooks to merchant sites upon payment confirmation.  
- **Security Layer:** HMAC/JWT verification for secure API and webhook handling.  
- **Data Storage:** PostgreSQL or SQLite for storing merchants, orders, and payments.  
- **Modular Architecture:** Internal crates for reusable functionality (monero-rpc, price-engine, security-utils).

---

## Architecture

The backend is built in **Rust**, using:

- **Async Runtime:** `tokio`
- **Web Framework:** `axum` (HTTP API)
- **Database:** `sqlx` (async, compile-time checked queries)
- **RPC Client:** `monero-rpc` crate or minimal typed RPC structs via `reqwest`
- **Security:** `hmac`, `jsonwebtoken`
- **Logging & Metrics:** `tracing`, `tracing-subscriber`, `prometheus`
- **Testing:** Unit, integration, and end-to-end tests


---

## Setup

1. **Clone the repository**

```bash
git clone <repo-url>
cd xmrbridge

2. **Install Rust toolchain**

```bash
rustup update stable

3. **Configure environment variables**

Create a .env file in gateway/:

```env
DATABASE_URL=postgres://user:password@localhost/xmrbridge
MONERO_RPC_URL=http://127.0.0.1:18082/json_rpc
MONERO_RPC_USER=rpc_user
MONERO_RPC_PASSWORD=rpc_password


---

## Running The Service

```bash
./scripts/reset_db.sh
./scripts/seed_db.sh

Start backend service

``` bash
cargo run -p gateway

The HTTP API will be available at http://127.0.0.1:8080.

---

## Database Migrations

All migrations are in migrations/. Example:

0001_init.sql → creates tables for merchants, orders, and payments.

Use sqlx CLI for applying migrations:

```bash
cargo install sqlx-cli
sqlx database setup
sqlx migrate run

---

## Testing

Unit Tests: cargo test -p gateway

Integration Tests: cargo test --test integration_invoices

End-to-End Tests: cargo test --test e2e_checkout

Tests are organized per module and also include top-level E2E scenarios simulating checkout and payment confirmation.

---

## License

All rights reserved. This project is proprietary and owned by The Market Index LLC.
