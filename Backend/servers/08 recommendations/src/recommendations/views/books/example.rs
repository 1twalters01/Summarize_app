use crate::{
    generated::protos::recommendations::books::example::{
        request::Request,
        response::{response::ResponseField, BookRecommendation, Error, Response, Success},
    },
    utils::validations::{validate_book_id, validate_genre_level, validate_recommendation_number},
};
use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};
use pyo3::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub genres: Vec<String>,
}

pub async fn post_book_id(data: ProtoBuf<Request>) -> Result<impl Responder> {
    // get request variables
    let Request {
        book_id,
        genre_level,
        recommendation_number,
    } = data.0;

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

    let recommendations: Result<Vec<BookRecommendation>, ()> =
        get_recommendations(book_id, genre_level, recommendation_number);
    match recommendations {
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

pub fn get_recommendations(
    book_id: String,
    genre_level: i32,
    recommendation_number: i32,
) -> Result<Vec<BookRecommendation>, ()> {
    pyo3::prepare_freethreaded_python();

    let py_code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/recommendations/algorithms/books/example.py"
    ));

    return Python::with_gil(|py| {
        let py_engine = PyModule::from_code_bound(
            py,
            py_code,
            "recommendations.algorithms.books.example",
            "recommendations.algorithms.books.example",
        )
        .map_err(|err| format!("Failed to import Python module: {:?}", err))
        .unwrap();

        let py_fun = py_engine
            .getattr("get_recommendations")
            .map_err(|err| format!("Failed to get Python Function: {:?}", err))
            .unwrap();

        let py_recommendations = py_fun
            .call1((book_id, genre_level, recommendation_number))
            .map_err(|err| format!("Python function call failed: {:?}", err))
            .unwrap()
            .to_string();

        let books: Vec<Recommendation> = serde_json::from_str(&py_recommendations).unwrap();
        let recommendations = books
            .into_iter()
            .map(|recommendation| BookRecommendation {
                id: recommendation.id,
                title: recommendation.title,
                authors: recommendation.authors,
                genres: recommendation.genres,
            })
            .collect::<Vec<BookRecommendation>>();
        Ok::<Vec<BookRecommendation>, ()>(recommendations)
    });
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use bytes::Bytes;
    use prost::{DecodeError, Message};

    use crate::generated::protos::recommendations::books::example::{
        request::Request, response::Response,
    };

    use super::post_book_id;

    #[actix_web::test]
    async fn test_invalid() {
        let mut app = test::init_service(App::new().service(
            web::scope("/recommendations").route("/example", web::post().to(post_book_id)),
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
            .uri("/recommendations/example")
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
