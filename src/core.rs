/*
 * Copyright (c) 2024 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::collections::HashSet;

use actix_web::{get, web::Path, HttpResponse, Responder};
use log::debug;

#[get("{scope}/{url}")]
pub async fn coniunctio(path: Path<(String, String)>) -> impl Responder {
    debug!("Request: {path:?}");
    HttpResponse::Ok()
}

#[derive(PartialEq, Eq, Hash)]
pub enum Policies {
    ForbidExtern,
    WarnExtern,
    AllowRedirects,
    AllowTrees,
    ForbidCustomUrl,
    ForbidUserCreation,
    EnforceUniqueness,
    EnforceLogin,
}
