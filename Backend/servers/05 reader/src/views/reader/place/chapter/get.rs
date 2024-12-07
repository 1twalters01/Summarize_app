// Get/set chapter

async pub fn post_read_summary_chapter(data: Protobuf<request>) -> Result<impl Responder> {
    // chapter id is a uuid
    // update is a bool that says if this going to update the last read chapter by the user
    // chapter_uuid_option is option<chapter_uuid>
    let Request { summary_id, chapter_uuid_option, update } = data.0;

    // Validate all three

    // if chapter_uuid = none then
        // get last_chapter_id where summary_id = summary_id
        // if none then get the chapter id for cover
    
    // check cache, return if success
    // get chapter from s3
    // save to cache

    // if update = true then update last_chapter_id
    // return
}
