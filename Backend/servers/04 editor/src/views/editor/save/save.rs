use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};

// Save summary changes
pub async fn post_save(data: ProtoBuf<Request>) -> Result<impl Responder> {
    // get request variables
    let Request { summary } = data.0;

    // Validate the summary
    let validated_summary = validate_book_id(&summary);
    if validated_summary.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::InvalidSummary as i32)),
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // save to s3
    // return ok
}
