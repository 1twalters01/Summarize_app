//

async pub fn delete_highlight(data: Protobuf<request>) -> Result<impl Responder> {
    // get request variables
    let Request { chapter_id } = data.0;
}