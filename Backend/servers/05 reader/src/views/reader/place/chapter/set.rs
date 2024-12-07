async pub fn post_update_summary_chapter(data: Protobuf<request>) -> Result<impl Responder> {
    let Request { summary_id, chapter_id } = data.0;

    // Validate
}
