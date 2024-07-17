pub async fn get_favourite_genres(data: Json<FavouriteGenresRequestStruct>) -> Result<impl Responder> {
  // postgres request for genres by userid, sorted by ascending rank order
  // limit by minimum of the number of genres or the upper limit of x.

  let FavouriteGenresRequestSchema { number_of_genres } = data.into_inner();
  let mut res_body: FavouriteGenresResponseSchema = FavouriteGenresResponseSchema::new();

  // Error if number_of_genres > 15?
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
    let genres_result: Result<Option<Vec<String>>, sqlx::Error> = get_favourite_genres_from_uuid(user_uuid, upper_limit);
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

pub async fn get_favourite_genres_from_uuid(pool: &Pool<Postgres>, user_uuid: String, upper_limit: u8) -> Result<Option<Vec<String>>, sqlx::Error> {
    let genre_select_query = sqlx::query("Select * from genres WHERE uuid=($1) limit=($2)")
        .bind(Uuid::from_string(user_uuid))
        .bind(upper_limit)
        .fetch_all(pool);

    match genre {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 { return Ok(None) }

            let genres: Vec<String> = res[0].get_all();
            return Ok(Some(genres));
        }
    };
}


