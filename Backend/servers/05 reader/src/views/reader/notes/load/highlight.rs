// highlight locations and highlight types

async pub fn load_highlights(data: Protobuf<request>) -> Result<impl Responder> {
    // get request variables
    let Request { chapter_id } = data.0;
}