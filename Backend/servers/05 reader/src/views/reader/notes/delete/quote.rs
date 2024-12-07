//

async pub fn delete_quote(data: Protobuf<request>) -> Result<impl Responder> {
    // get request variables
    let Request { chapter_id } = data.0;
}