use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, Responder, Result};

async pub fn increment_summary_chapter(data: Protobuf<request>) -> Result<impl Responder> {
    let Request { summary_id } = data.0

    // Validate

    // check if incrementing summary is possible
    // if no then return error::LastChapter
    // if yes then update and then return result
}
