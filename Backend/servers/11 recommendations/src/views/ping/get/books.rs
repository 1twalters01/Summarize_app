use crate::{
    generated::protos::recommendations::books::example::response::{
        response::ResponseField,
        BookRecommendation,
        Error,
        Response,
        Success
    },
    services::ping::get::books::recommendations,
    utils::validations::{validate_book_id, validate_genre_level, validate_recommendation_number},
};
use actix_protobuf::ProtoBufResponseBuilder;
use actix_web::{HttpResponse, Responder, Result};

pub async fn get_books() -> Result<impl Responder> {
    // get request variables
    let book_id = "2a9089f2-01c9-4f97-8f3e-69e3a5fcd04d".to_string();
    let genre_level = 1;
    let recommendation_number = 5;

    // Validate variables from the request body
    let validated_book_id = validate_book_id(&book_id);
    if validated_book_id.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::InvalidBookId as i32)),
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    let validated_genre_level = validate_genre_level(genre_level);
    if validated_genre_level.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::InvalidGenreLevel as i32)),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    let validated_recommendation_number = validate_recommendation_number(recommendation_number);
    if validated_recommendation_number.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(
                Error::InvalidRecommendationNumber as i32,
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    let book_recommendations: Result<Vec<BookRecommendation>, ()> =
        recommendations(book_id, genre_level, recommendation_number);

    match book_recommendations {
        Ok(recommendations) => {
            let response: Response = Response {
                response_field: Some(ResponseField::Success(Success { recommendations })),
            };
            return Ok(HttpResponse::Ok()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Err(err) => {
            println!("err: {:#?}", err);
            let response: Response = Response {
                response_field: Some(ResponseField::Error(Error::ServerError as i32)),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use bytes::Bytes;
    use prost::{DecodeError, Message};

    use crate::generated::protos::recommendations::books::example::{
        request::Request, response::Response,
    };

    use super::*;

    #[actix_web::test]
    async fn test_invalid() {
        let mut app = test::init_service(App::new().service(
            web::scope("/ping").route("/post_books", web::post().to(get_books)),
        ))
        .await;

        let book_id = "2a9089f2-01c9-4f97-8f3e-69e3a5fcd04d".to_string();
        let genre_level = 1;
        let recommendation_number = 5;
        let req_message = Request {
            book_id,
            genre_level,
            recommendation_number,
        };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let request = test::TestRequest::post()
            .uri("/ping/post_books")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded_result: Result<Response, DecodeError> = Message::decode(&response_buffer[..]);

        match decoded_result {
            Ok(_) => assert!(true),
            Err(err) => {
                println!("err: {:#?}", err);
                panic!("");
            }
        }
    }
}

