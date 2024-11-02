use actix_web::{HttpResponse, Responder, Result, web::Bytes};
use std::{fs, path::PathBuf, env};

pub async fn post_specific() -> Result<impl Responder> {
    // retrieve specific c files version /some uuid representation of them
    // return the diff that brings it to the new version
}

pub async fn post_push() -> Result<impl Responder> {
    // post most up to date versions of the c files
    // calculate the diff from the previous version and save
    // respond ok or err
}

