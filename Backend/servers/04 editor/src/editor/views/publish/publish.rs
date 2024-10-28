use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};

// Toggle published status of summary
async pub fn post_toggle_published_status(data: ProtoBuf<Request>) {
    // get request variable
    let Request { new_published_status } = data.0;

    // Check user is permitted else return error
    // Set published status to new_published_status
    // return ok
}
