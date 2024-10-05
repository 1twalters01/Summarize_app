use std::str::FromStr;

use uuid::Uuid;

pub fn validate_book_id(book_id: &str) -> Result<(), String> {
    match Uuid::from_str(book_id) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err.to_string()),
    }
}
pub fn validate_genre_level(genre_level: i32) -> Result<(), String> {
    match genre_level < 20 {
        true => return Ok(()),
        false => return Err("Number too large".to_string()),
    }
}
pub fn validate_recommendation_number(recommendation_number: i32) -> Result<(), String> {
    match recommendation_number < 20 {
        true => return Ok(()),
        false => return Err("Number too large".to_string()),
    }
}
