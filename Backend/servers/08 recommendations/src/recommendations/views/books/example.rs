use std::collections::HashMap;

use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};
use pyo3::{prelude::*, types::{PyDict, PyTuple}, Py, PyAny};
use serde::Deserialize;
use crate::{
    generated::protos::recommendations::books::example::{
        request::Request,
        response::{response::ResponseField, Error, Response, Success},
    },
    utils::validations::{validate_book_id, validate_genre_level, validate_recommendation_number}
};

#[derive(Debug)]
struct BookRecommendations {
    id: String,
    title: String,
    authors: Vec<String>,
    genres: Vec<String>
}

pub async fn post_book_id(data: ProtoBuf<Request>) -> Result<impl Responder> {
    // get request variable
    let Request { book_id, genre_level, recommendation_number } = data.0;

    // Validate variables from the request body
    // let validated_book_id = validate_book_id(&book_id);
    // if validated_book_id.is_err() {
    //     let response: Response = Response {
    //         response_field: Some(ResponseField::Error(Error::InvalidBookId as i32)),
    //     };
    //
    //     return Ok(HttpResponse::UnprocessableEntity()
    //         .content_type("application/x-protobuf; charset=utf-8")
    //         .protobuf(response));
    // }
    // let validated_genre_level = validate_genre_level(genre_level);
    // if validated_genre_level.is_err() {
    //     let response: Response = Response {
    //         response_field: Some(ResponseField::Error(Error::InvalidGenreLevel as i32)),
    //     };
    //
    //     return Ok(HttpResponse::UnprocessableEntity()
    //         .content_type("application/x-protobuf; charset=utf-8")
    //         .protobuf(response));
    // }
    // let validated_recommendation_number = validate_recommendation_number(recommendation_number);
    // if validated_recommendation_number.is_err() {
    //     let response: Response = Response {
    //         response_field: Some(ResponseField::Error(Error::InvalidRecommendationNumber as i32)),
    //     };
    //
    //     return Ok(HttpResponse::UnprocessableEntity()
    //         .content_type("application/x-protobuf; charset=utf-8")
    //         .protobuf(response));
    // }

    let recommendations: Result<Vec<BookRecommendations>, ()> = get_recommendations(book_id, genre_level, recommendation_number);
    match recommendations {
        Ok(recommendations) => {
            let response: Response = Response {
                response_field: Some(ResponseField::Success(Success {}))
            };
            return Ok(HttpResponse::Ok()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        },
        Err(err) => {
            let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::ServerError as i32)),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    }
}

pub fn get_recommendations(book_id: String, genre_level: i32, recommendation_number: i32) -> Result<Vec<BookRecommendations>, ()> {
    let arg1 = "arg1";
    let arg2 = "arg2";
    let arg3 = "arg3";

    pyo3::prepare_freethreaded_python();
    
    // Python::with_gil(|py| {
    //     let fun: Py<PyAny> = PyModule::from_code_bound(
    //         py,
    //         "def example(*args, **kwargs):
    //             if args != ():
    //                 print('called with args', args)
    //             if kwargs != {}:
    //                 print('called with kwargs', kwargs)
    //             if args == () and kwargs == {}:
    //                 print('called with no arguments')",
    //         "",
    //         "",
    //     ).unwrap()
    //     .getattr("example").unwrap()
    //     .into();
    //     println!("{:#?}", fun);
    //
    //     // call object without any arguments
    //     fun.call0(py).unwrap();
    //
    //     // pass object with Rust tuple of positional arguments
    //     let args = (arg1, arg2, arg3);
    //     fun.call1(py, args).unwrap();
    //
    //     // call object with Python tuple of positional arguments
    //     let args = PyTuple::new_bound(py, &[arg1, arg2, arg3]);
    //     fun.call1(py, args).unwrap();
    // });

    let py_code = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/recommendations/algorithms/books/example.py"
            ));
    // println!("{:?}", py_code);

    Python::with_gil(|py| {
        let py_engine = PyModule::from_code_bound(py, py_code, "recommendations.algorithms.books.example", "recommendations.algorithms.books.example")
            .map_err(|err| format!("Failed to import Python module: {:?}", err))
            .unwrap();

        let py_fun = py_engine
            .getattr("get_recommendations")
            .map_err(|err| format!("Failed to get Python Function: {:?}", err))
            .unwrap();

        let py_recommendations = py_fun
            .call1((book_id, genre_level, recommendation_number))
            .map_err(|err| format!("Python function call failed: {:?}", err))
            .unwrap().to_string();
        // println!("py_recommendations: {:#?}", py_recommendations);

        #[derive(Debug, Deserialize)]
        struct Book {
            id: String,
            title: String,
            authors: Vec<String>,
            genres: Vec<String>,
        }

        // use extract instead of serde for speed?
        let books: Vec<Book> = serde_json::from_str(&py_recommendations).unwrap();
        println!("Books: {:#?}", books);
        // let recommendations = py_recommendations
        //     .extract::<Vec<HashMap<String, Vec<String> >>>()
        //     .unwrap();
        // println!("recommendations: {:#?}", recommendations);

    //     let recommendations = py_recommendations
    //         .extract::<Vec<PyObject>>()
    //         .map_err(|err| format!("Failed to extract Python object: {:?}", err))?;
    //
    //     let recommendations: Vec<BookRecommendations> = recommendations
    //         .into_iter()
    //         .map(|py_obj| {
    //             let py_any: PyDict = py_obj.as_any().into()?;
    //             
    //             Ok(BookRecommendations {
    //                 id: py_dict.get_item("id").unwrap().extract::<String>()?,
    //                 title: py_dict.get_item("title").unwrap().extract::<String>()?,
    //                 authors: py_dict.get_item("authors").unwrap().extract::<Vec<String>>()?,
    //                 genres: py_dict.get_item("genres").unwrap().extract::<Vec<String>>()?,
    //             })
    //         })
    //         .collect::<Result<Vec<BookRecommendations>, _>>()?;
    //     return Ok(recommendations)
    });
    // return Err(());

    let book_recommendations: Vec<BookRecommendations> = vec![
        BookRecommendations {
            id: "aaaa".to_string(),
            title: "The Art of Electronics".to_string(),
            authors: vec!["Paul Horowitz".to_string(), "Winfield Hill".to_string()],
            genres: vec!["Electrical Engineering".to_string(), "Engineering".to_string(), "Non-fiction".to_string()],
        }
    ];
    // println!("{:#?}", book_recommendations);
    return Ok(book_recommendations)
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use bytes::Bytes;
    use prost::Message;

    use crate::generated::protos::recommendations::books::example::{request::Request, response::Response};

    use super::post_book_id;

    #[actix_web::test]
    async fn test_invalid() {
        let mut app = test::init_service(
            App::new().service(
                web::scope("/recommendations")
                .route("/example", web::post().to(post_book_id)),
            ),
        )
        .await;

        let book_id = "cccc".to_string();
        let genre_level = 1;
        let recommendation_number = 5;
        let req_message = Request { book_id, genre_level, recommendation_number };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let request = test::TestRequest::post()
            .uri("/recommendations/example")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();

        panic!("boo")
    }
}
