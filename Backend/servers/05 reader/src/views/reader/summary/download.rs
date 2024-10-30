// download the summary to one of many formats

async pub fn post_download_summary(data: Protobuf<request>) -> Result<impl Responder> {
    // get request variables
    let Request { summary_id, format } = data.0;

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

    // Validate the variables from the request body
    let validated_format = validate_book_id(&format);
    if validated_format.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::InvalidFormat as i32)),
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }
    
    // Check redis for the summary_id for downloads
    // error if error
    // if uncussessful then
        // check pg for summary s3 address
        // fetch from s3 if success else error
    // Convert to format using polyglot
    // return error if error
    // return file 
}