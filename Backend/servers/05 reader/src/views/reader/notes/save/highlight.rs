// highlight locations and highlight types

async pub fn save_highlight(data: Protobuf<request>) -> Result<impl Responder> {
    // get request variables
    let Request { chapter_id } = data.0;
}