use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, Responder, Result};

pub async fn update_cached_summaries() -> Result<impl Responder> {
}

#[cfg(test)]
mod tests {
    use super::*;
}
