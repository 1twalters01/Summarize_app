use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, Responder, Result};

pub async fn get_cached_books() -> Result<impl Responder> {
}

#[cfg(test)]
mod tests {
    use super::*;
}
