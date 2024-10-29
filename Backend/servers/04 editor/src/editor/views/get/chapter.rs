use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};

// Get Chapter being worked on
pub async fn post_retrieve_chapter(data: Protobuf<Request>) -> Result<impl Responder> {
    // get request variables
    let Request { summary_id, chapter_id } = data.0;

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

    // Validate the chapter_id from the request body
    let validated_chapter_id = validate_email(&chapter_id);
    if validated_chapter_id.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::InvalidChapterId as i32)),
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // Check redis if summary_id, chapter_id returns a chapter
    // Check user is permitted
    // if yes to both then return chapter

    // Look up S3 location and if user is permitted using summary_id and chapter_id in postgres
    // if user is not permitted then return error
    // if location is invalid then return error

    // Get chapter from S3
    // return if error
    // Save to redis
    // return if error

    // return chapter
}
