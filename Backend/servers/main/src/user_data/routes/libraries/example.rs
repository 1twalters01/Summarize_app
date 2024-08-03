use actix_web::{web::Json, HttpResponse, Responder, Result};
use serde::{Serialize, Deserialize};
use std::cmp;

#[derive(Serialize, Deserialize)]
pub struct ExampleLibrariesRequest {
  pub number_of_libraries: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ExampleLibrariesResponse {
    pub libraries: Vec<Library>,
}

impl ExampleLibrariesResponse {
  pub fn new() -> ExampleLibrariesResponse {
    ExampleLibrariesResponse {
        libraries: Vec::new()
    }
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Library {
    pub name: String,
    pub number_of_books: u64,
}

impl Library {
    pub fn from(name: String, number_of_books: u64) -> Library {
        Library {
            name,
            number_of_books,
        }
    }
}

pub async fn get_libraries(data: Json<ExampleLibrariesRequest>) -> Result<impl Responder> {
  let ExampleLibrariesRequest { number_of_libraries } = data.into_inner();
  let mut res_body: ExampleLibrariesResponse = ExampleLibrariesResponse::new();
  
  let request_limit = 15;
  let upper_limit = cmp::min(request_limit, number_of_libraries);

  let library_arr: [Library; 15] = [
      Library::from("Robert Greene".to_string(), 8),
      Library::from("Computer Science".to_string(), 12),
      Library::from("Human Nature".to_string(), 3),
      Library::from("Self help".to_string(), 36),
      Library::from("Engineering".to_string(), 13),
      Library::from("CAD".to_string(), 12),
      Library::from("Fiction".to_string(), 9),
      Library::from("First Aid".to_string(), 7),
      Library::from("Nutrition".to_string(), 5),
      Library::from("Horror".to_string(), 3),
      Library::from("Marketing".to_string(), 17),
      Library::from("Sales".to_string(), 24),
      Library::from("Statistics".to_string(), 2),
      Library::from("Piano".to_string(), 14),
      Library::from("Chess".to_string(), 8),
  ];

  let library_vec:Vec<Library> = library_arr[0..upper_limit as usize]
      .iter()
      .map(|library| library.clone())
      .collect();

  res_body.libraries = library_vec;
  return Ok(HttpResponse::Ok()
      .content_type("application/json; charset=utf-8")
      .json(res_body));
}

