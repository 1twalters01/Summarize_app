use crate::{
    generated::protos::recommendations::books::example::response::BookRecommendation,
    datatypes::ping::Recommendation,
};
use pyo3::prelude::*;

pub fn books(
    book_id: String,
    genre_level: i32,
    recommendation_number: i32,
) -> Result<Vec<BookRecommendation>, ()> {
    pyo3::prepare_freethreaded_python();

    let py_code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/scripts/ping/books.py"
    ));

    return Python::with_gil(|py| {
        let py_engine = PyModule::from_code_bound(
            py,
            py_code,
            "scripts.ping.books",
            "scripts.ping.books",
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
