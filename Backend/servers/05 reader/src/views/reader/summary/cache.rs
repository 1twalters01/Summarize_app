// Choose summary to be read

async pub fn post_summary_selector(data: Protobuf<request>) -> Result<impl Responder> {
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

    // load chapter to cache
    // return ok
}