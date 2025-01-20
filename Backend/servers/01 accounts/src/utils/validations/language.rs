use crate::generated::protos::settings::profile::language::request::Language;

pub fn validate_language(language: i32) -> Result<(), String> {
    match Language::try_from(language) {
        Ok(_) => return Ok(()),
        Err(error) => return Err(error.to_string()),
    }
}
