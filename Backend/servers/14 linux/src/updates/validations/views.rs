use actix_web::{HttpResponse, Responder, Result, web::Bytes};
use std::{fs, path::PathBuf, env};

pub async fn check() -> Result<impl Responder> {
    // post lua last update version, lua last update time, c last update version, c last update time
    // return [lua: bool, c: bool], true if up to date else false 
}

pub async fn validate() -> Result<impl Responder> {
    // send list of all files hashes in order of file size for lua and for c
    // if correct then return true else return false
}
