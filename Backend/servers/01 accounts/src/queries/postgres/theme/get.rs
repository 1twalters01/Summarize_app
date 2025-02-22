use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::generated::protos::settings::profile::theme::request::{
    request::RequestField, Colour, Colours, Custom, Presets, Theme,
};

pub async fn get_theme_from_uuid(
    pool: &Pool<Postgres>,
    user_uuid: &Uuid,
) -> Result<Theme, sqlx::Error> {
}