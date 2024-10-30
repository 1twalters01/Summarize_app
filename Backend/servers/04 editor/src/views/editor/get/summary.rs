use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};

// Get summary data
pub async fn post_retrieve_summary(data: Protobuf<Request>) -> Result<impl Responder> {
    // get request variables
    let Request { summary_id } = data.0;

    // Validate the variables from the request body
    let validated_summary_id = validate_book_id(&summary_id);
    if validated_summary_id.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::InvalidSummaryId as i32)),
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // Check redis if summary_id returns data
    // Check user is permitted
    // if yes to both then return data

    // Look up S3 location and if user is permitted using summary_id in postgres
    // if location is invalid then return error

    // Get summary data from S3
    // return if error
    // Save to redis
    // return if error

    // return chapter
}
