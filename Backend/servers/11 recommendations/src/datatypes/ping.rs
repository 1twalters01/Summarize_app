use pyo3::FromPyObject;
use serde::Deserialize;

#[derive(Debug, Deserialize, FromPyObject)]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub genres: Vec<String>,
}
