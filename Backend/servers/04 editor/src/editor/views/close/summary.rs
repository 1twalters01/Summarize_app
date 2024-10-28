use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};

// close summary (remove from redis), save or not save option

async pub fn close_summary(data: Protobuf<Request>) {
    
}