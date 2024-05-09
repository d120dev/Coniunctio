/*
 * Copyright (c) 2024 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use coniunctio::coniunctio_run;

#[actix_web::main]
async fn main() {
    coniunctio_run().await
}
