use crate::{
    settings::schema::SettingsError, utils::database_connections::create_pg_pool_connection,
};
use actix_web::{post, web::Json, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(PartialEq, Serialize, Deserialize)]
pub enum PreferredLength {
    None,
    VShort, // < 5 min total read
    Short,  // 5 <= x < 15 min total read
    Medium, // 15 <= x 30 min total read
    Long,   // 30 min <= 1 hour total read
    VLong,  // > 1 hour total read
}

impl PreferredLength {
    fn to_string(&self) -> String {
        match self {
            Self::None => return "".to_string(),
            Self::VShort => return "VShort".to_string(),
            Self::Short => return "Short".to_string(),
            Self::Medium => return "Medium".to_string(),
            Self::Long => return "Long".to_string(),
            Self::VLong => return "VLong".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChangeRoughPreferredLengthRequestStruct {
    pub preferred_lengths: PreferredLengths,
}

#[derive(Serialize, Deserialize)]
pub struct PreferredLengths {
    primary: Option<PreferredLength>,
    secondary: Option<PreferredLength>,
    tertiary: Option<PreferredLength>,
    quaternary: Option<PreferredLength>,
    quinternary: Option<PreferredLength>,
}

impl PreferredLengths {
    fn get_primary(&self) -> String {
        if let Some(primary) = &self.primary {
            return primary.to_string();
        }

        return String::new();
    }

    fn get_secondary(&self) -> String {
        if let Some(secondary) = &self.secondary {
            return secondary.to_string();
        }

        return String::new();
    }

    fn get_tertiary(&self) -> String {
        if let Some(tertiary) = &self.tertiary {
            return tertiary.to_string();
        }

        return String::new();
    }

    fn get_quaternary(&self) -> String {
        if let Some(quaternary) = &self.quaternary {
            return quaternary.to_string();
        }

        return String::new();
    }

    fn get_quinternary(&self) -> String {
        if let Some(quinternary) = &self.quinternary {
            return quinternary.to_string();
        }

        return String::new();
    }
}

#[derive(Serialize)]
pub struct ChangeRoughPreferredLengthResponseStruct {
    settings_error: SettingsError,
    preferred_lengths: Option<PreferredLengths>,
}

impl ChangeRoughPreferredLengthResponseStruct {
    pub fn new() -> ChangeRoughPreferredLengthResponseStruct {
        ChangeRoughPreferredLengthResponseStruct {
            settings_error: SettingsError::new(),
            preferred_lengths: None,
        }
    }
}

#[post("change_rough_prefered_lengths")]
pub async fn change_rough_preferred_lengths(
    req_body: Json<ChangeRoughPreferredLengthRequestStruct>,
    // req: HttpRequest,
) -> Result<impl Responder> {
    let ChangeRoughPreferredLengthRequestStruct { preferred_lengths } = req_body.into_inner();
    let mut res_body: ChangeRoughPreferredLengthResponseStruct =
        ChangeRoughPreferredLengthResponseStruct::new();

    // Authenticate, is this done outside of this function?

    // Validate preferred lengths else error
    let validated_preferred_lengths = validate_preferred_lengths(&preferred_lengths);
    if validated_preferred_lengths.is_err() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_preferred_lengths.err().unwrap()),
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // Ensure password is correct for user

    // Change preferred length
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_rough_preferred_lengths_for_user_in_pg_content_delivery_table(
            &pool,
            &preferred_lengths,
        )
        .await;

    // if sql update error then return an error
    if update_result.is_err() {
        res_body.settings_error = SettingsError {
            is_error: true,
            error_message: Some(String::from("internal error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

fn validate_preferred_lengths(preferred_lengths: &PreferredLengths) -> Result<(), String> {
    let PreferredLengths {
        primary,
        secondary,
        tertiary,
        quaternary,
        quinternary,
    } = preferred_lengths;
    match (primary, secondary, tertiary, quaternary, quinternary) {
        (None, None, None, None, None) => return Ok(()),
        (_, _, _, None, Some(_)) => return Err(String::from("null condition not met")),
        (_, _, None, Some(_), _) => return Err(String::from("null condition not met")),
        (_, None, Some(_), _, _) => return Err(String::from("null condition not met")),
        (None, Some(_), _, _, _) => return Err(String::from("null condition not met")),

        (Some(p), Some(s), Some(t), Some(qua), Some(qui))
            if p == s || p == t || p == qua || p == qui =>
        {
            return Err(String::from("Repeat values found"))
        }
        (_, Some(s), Some(t), Some(qua), Some(qui)) if s == t || s == qua || s == qui => {
            return Err(String::from("Repeat values found"))
        }
        (_, _, Some(t), Some(qua), Some(qui)) if t == qua || t == qui => {
            return Err(String::from("Repeat values found"))
        }
        (_, _, _, Some(qua), Some(qui)) if qua == qui => {
            return Err(String::from("Repeat values found"))
        }
        (_, _, _, _, Some(_)) => return Ok(()),
        (Some(_), None, None, None, None) => return Ok(()),
        (Some(_), Some(_), None, None, None) => return Ok(()),
        (Some(_), Some(_), Some(_), None, None) => return Ok(()),
        (Some(_), Some(_), Some(_), Some(_), None) => return Ok(()),
    }
}

pub async fn update_rough_preferred_lengths_for_user_in_pg_content_delivery_table(
    pool: &Pool<Postgres>,
    preferred_lengths: &PreferredLengths,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("")
        .bind(preferred_lengths.get_primary())
        .bind(preferred_lengths.get_secondary())
        .bind(preferred_lengths.get_tertiary())
        .bind(preferred_lengths.get_quaternary())
        .bind(preferred_lengths.get_quinternary())
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum DeliveryMethod {
    Text,
    Audio,
    Video,
}

impl DeliveryMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Text => return "Text".to_string(),
            Self::Audio => return "Audio".to_string(),
            Self::Video => return "Video".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct ChangeDeliveryMethodRequestStruct {
    delivery_methods: DeliveryMethods,
}

#[derive(Serialize, Deserialize)]
pub struct DeliveryMethods {
    primary: Option<DeliveryMethod>,
    secondary: Option<DeliveryMethod>,
    tertiary: Option<DeliveryMethod>,
}

impl DeliveryMethods {
    fn get_primary(&self) -> String {
        if let Some(primary) = &self.primary {
            return primary.to_string();
        }

        return String::new();
    }

    fn get_secondary(&self) -> String {
        if let Some(secondary) = &self.secondary {
            return secondary.to_string();
        }

        return String::new();
    }

    fn get_tertiary(&self) -> String {
        if let Some(tertiary) = &self.tertiary {
            return tertiary.to_string();
        }

        return String::new();
    }
}

#[derive(Serialize)]
pub struct ChangeDeliveryMethodResponseStruct {
    settings_error: SettingsError,
    delivery_methods: Option<DeliveryMethods>,
}

impl ChangeDeliveryMethodResponseStruct {
    pub fn new() -> ChangeDeliveryMethodResponseStruct {
        ChangeDeliveryMethodResponseStruct {
            settings_error: SettingsError::new(),
            delivery_methods: None,
        }
    }
}

#[post("change_delivery_methods")]
async fn change_delivery_methods(
    req_body: Json<ChangeDeliveryMethodRequestStruct>,
    // req: HttpRequest,
) -> Result<impl Responder> {
    let ChangeDeliveryMethodRequestStruct { delivery_methods } = req_body.into_inner();
    let mut res_body: ChangeDeliveryMethodResponseStruct =
        ChangeDeliveryMethodResponseStruct::new();

    // Authenticate, is this done outside of this function?

    // Validate delivery methods else error
    let validated_delivery_methods = validate_delivery_methods(&delivery_methods);
    if validated_delivery_methods.is_err() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_delivery_methods.err().unwrap()),
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // Ensure password is correct for user

    // Change preferred length
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_delivery_methods_for_user_in_pg_content_delivery_table(&pool, &delivery_methods)
            .await;

    // if sql update error then return an error
    if update_result.is_err() {
        res_body.settings_error = SettingsError {
            is_error: true,
            error_message: Some(String::from("internal error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

pub fn validate_delivery_methods(delivery_methods: &DeliveryMethods) -> Result<(), String> {
    let DeliveryMethods {
        primary,
        secondary,
        tertiary,
    } = delivery_methods;
    // Ensure that no value is repeated in ChangeDeliveryMethodStruct if not none
    if primary == secondary || primary == tertiary || secondary == tertiary {
        return Err(String::from("Repeat values found"));
    }

    return Ok(());
}

pub async fn update_delivery_methods_for_user_in_pg_content_delivery_table(
    pool: &Pool<Postgres>,
    delivery_methods: &DeliveryMethods,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("")
        .bind(delivery_methods.get_primary())
        .bind(delivery_methods.get_secondary())
        .bind(delivery_methods.get_tertiary())
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}
