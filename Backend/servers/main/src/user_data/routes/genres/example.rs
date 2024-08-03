use actix_web::{web::Json, HttpResponse, Responder, Result};
use serde::{Serialize, Deserialize};
use std::cmp;

#[derive(Serialize, Deserialize)]
pub struct ExampleGenresRequest {
  pub number_of_genres: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ExampleGenresResponse {
    pub genres: Vec<String>,
}

impl ExampleGenresResponse {
  pub fn new() -> ExampleGenresResponse {
    ExampleGenresResponse {
        genres: Vec::new()
    }
  }
}

pub async fn get_genres(data: Json<ExampleGenresRequest>) -> Result<impl Responder> {
  let ExampleGenresRequest { number_of_genres } = data.into_inner();
  let mut res_body: ExampleGenresResponse = ExampleGenresResponse::new();
  
  let request_limit = 15;
  let upper_limit = cmp::min(request_limit, number_of_genres);

  let genre_arr: [&str; 15] =
    ["Fantasy", "Science fiction", "Dystopian Fiction", "Horror", "Mystery",
     "History", "Engineering", "Thriller", "Graphic novel", "Philosophy",
     "Self-help", "Psychology", "Autobiographies", "Design", "Business"];

  let genre_vec:Vec<String> = genre_arr[0..upper_limit as usize]
      .iter()
      .map(|genre| String::from(*genre))
      .collect();

  res_body.genres = genre_vec;
  return Ok(HttpResponse::Ok()
      .content_type("application/json; charset=utf-8")
      .json(res_body));
}
