use actix_web::{web::Json, HttpRequest, HttpResponse, Responder, Result};
use Uuid;
use std::cmp;

pub struct FavouriteGenresRequestSchema {
  pub number_of_genres: u8,
}

pub struct FavouriteGenresResponseSchema {
}

impl FavouriteGenresResponseSchema {
  pub fn new() -> FavouriteGenresResponseSchema {
    FavouriteGenresResponseSchema {
    }
  }

pub async fn get_example_genres(data: Json<FavouriteGenresRequestStruct>) -> Result<impl Responder> {
  let FavouriteGenresRequestSchema { number_of_genres } = data.into_inner();
  let mut res_body: FavouriteGenresResponseSchema = FavouriteGenresResponseSchema::new();
  
  let request_limit = 15;
  let upper_limit = cmp::min(request_limit, number_of_genres);

  let genre_arr: [&str; 15] =
    ["Fantasy", "Science fiction", "Dystopian Fiction", "Horror", "Mystery",
     "History", "Engineering", "Thriller", "Graphic novel", "Philosophy",
     "Self-help", "Psychology", "Autobiographies", "Design", "Business"];
  let genre_vec:Vec<String> = genre_arr[0..upper_limit].iter().map(|s| String::from(*s)).collect();

  res_body.genres = genre_vec;
  return Ok(HttpResponse::Ok()
      .content_type("application/json; charset=utf-8")
      .json(res_body));
}

pub async fn summaries(data: Json<>) -> Result<impl Responder> {
  
}
