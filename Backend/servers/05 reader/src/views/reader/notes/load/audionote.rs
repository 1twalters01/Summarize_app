// load audionotes

async pub fn load_audionotes(data: Protobuf<request>) -> Result<impl Responder> {
    // get request variables
    let Request { chapter_id } = data.0;
}