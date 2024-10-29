// Get/set chapter

async pub fn post_read_summary_chapter(data: Protobuf<request>) -> Result<impl Responder> {
    // chapter id is an uuid, update is a bool
    // instead of chapter_id it could be an int representing the chapter
    let Request { summary_id, chapter_id_option, update } = data.0;

    // Validate

    // if chapter_id = none then
        // get last_chapter_id where summary_id = summary_id
        // if none then get the chapter id for cover
    
    // check cache, return if success
    // get chapter from s3
    // save to cache

    // if update = true then update last_chapter_id
    // return
}

async pub fn post_update_summary_chapter(data: Protobuf<request>) -> Result<impl Responder> {
    let Request { summary_id, chapter_id } = data.0;

    // Validate
}

async pub fn post_increment_summary_chapter(data: Protobuf<request>) -> Result<impl Responder> {
    let Request { summary_id } = data.0

    // Validate

    // check if incrementing summary is possible
    // if no then return error::LastChapter
    // if yes then update and then return result
}

async pub fn post_decrement_summary_chapter(data: Protobuf<request>) -> Result<impl Responder> {
    let Request { summary_id } = data.0

    // Validate

    // check if decrementing summary is possible
    // if no then return error::LastChapter
    // if yes then update and then return result
}