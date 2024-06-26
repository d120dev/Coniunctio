/*
 * Copyright (c) 2024 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use actix_web::{
    middleware::{Compress, Logger, NormalizePath, TrailingSlash},
    web::{scope, Data},
    App, HttpServer,
};
use dotenv::var;
use log::info;
use sqlx::{migrate, PgPool};

mod core;
mod management;

struct AppData {
    pub _db: PgPool,
}

pub async fn coniunctio_run() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("{}", build_info());

    let postgres_connection_string =
        var("DATABASE_URL").expect("env variable `DATABASE_URL` must be set");

    let postgres = PgPool::connect(&postgres_connection_string)
        .await
        .expect("cannot connect to PostgreSQL");
    info!("connected to database");

    migrate!("./migrations")
        .run(&postgres)
        .await
        .expect("database migrations failed");
    info!("applied migrations");

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(Data::new(AppData {
                _db: postgres.clone(),
            }))
            .service(scope("manage").configure(management::configure))
            .service(core::coniunctio)
    })
    .bind(("0.0.0.0", 8000))
    .unwrap_or_else(|_| panic!("could not bind to {}", 8000))
    .run()
    .await
    .expect("failed to run server");
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn build_info() -> String {
    let name = format!(
        "{}: Copyright (c) 2024 Jonathan \"Nath\" Schild. Licensed under the EUPL-1.2",
        env!("CARGO_PKG_NAME")
    );

    let version = format!(
        "Version: {}+{}.{} at {}",
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_HASH"),
        env!("BUILD_EPOCH"),
        env!("BUILD_DATE")
    );

    let rustc = format!("Rust Version: {}", env!("BUILD_RUSTC"));

    let repo = format!("Source Code: {}", env!("CARGO_PKG_REPOSITORY"));

    let max = *[name.len(), version.len(), rustc.len(), repo.len()]
        .iter()
        .max()
        .unwrap(); // returns None if iter is empty, which should never occur

    let w = max + 2;
    format!(
        "\n#{}#\n#{}#\n#  {name:w$}#\n#  {version:w$}#\n#  {rustc:w$}#\n#  {repo:w$}#\n#{1}#\n#{0}#",
        "=".repeat(w + 2),
        " ".repeat(w + 2)
    )
}
