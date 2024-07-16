use actix_web::{web::Json, HttpRequest, HttpResponse, Responder, Result};
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

pub async fn get_favourite_genres(data: Json<FavouriteGenresRequestStruct>) -> Result<impl Responder> {
  // postgres request for genres by userid, sorted by ascending rank order
  // limit by minimum of the number of genres or the upper limit of x.

  let FavouriteGenresRequestSchema { number_of_genres } = data.into_inner();
  let mut res_body: FavouriteGenresResponseSchema = FavouriteGenresResponseSchema::new();
  
  let request_limit = 15;
  let upper_limit = cmp::min(request_limit, number_of_genres);

  let user_uuid: String = match req.extensions().get::<Claims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            res_body.settings_error = SettingsError {
                is_error: true,
                error_message: Some(String::from("error")),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
    };

    // Make a Genre struct instead of using String?
    let genres_result: Result<Option<Vec<String>, sqlx::Error> = get_favourite_genres_from_uuid(user_uuid, upper_limit);
    let genre_vec: Vec<String> = match genres_result {
      Err(_) => {
        res_body._error = Error {
                error_message: Some(String::from("error")),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
      },
      Ok(genre_vec) => genre_vec,
    };

    res_body.genres = genre_vec;
    return Ok(HttpResponse::Ok()
      .content_type("application/json; charset=utf-8")
      .json(res_body));
}

pub async fn get_favourite_genres_from_uuid(user_uuid: String, upper_limit: u8) {
}
 
pub async fn summaries(data: Json<>) -> Result<impl Responder> {
  
}
