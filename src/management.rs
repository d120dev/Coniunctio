/*
 * Copyright (c) 2024 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

use crate::build_info;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(info);
}

#[get("info")]
async fn info() -> impl Responder {
    HttpResponse::Ok().body(format!("{}\n", build_info()))
}
