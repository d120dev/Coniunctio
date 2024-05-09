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

pub struct Scope {
    id: u32,
    url_prefix: String,
    policies: HashSet<Policies>,
}

pub struct ObjectCommon {
    id: u32,
    url: String,
    warn_extern: bool,
    enforce_login: bool,
}

pub struct TreeEntry {
    target: String,
}

pub enum Objects {
    Redirect {
        common: ObjectCommon,
        target: String,
    },
    LinkTree {
        common: ObjectCommon,
        tree_entries: Vec<String>,
    },
}

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
