use actix_web::{get, post, web::Json, HttpRequest, HttpResponse, Responder, Result};

pub enum prefered_length {
    None,
    VShort, // < 5 min total read
    Short, // 5 <= x < 15 min total read 
    Medium, // 15 <= x 30 min total read
    Long, // 30 min <= 1 hour total read
    VLong, // > 1 hour total read
}

pub struct ChangeRoughPreferedLengthRequestStruct {
    preferred_lengths: PreferredLengths,
    password: String,
}

pub struct PreferredLengths {
    primary: Option<content_delivery>,
    secondary: Option<content_delivery>,
    tertiary: Option<content_delivery>,
    quaternary: Option<content_delivery>,
    quinternary: Option<content_delivery>,
}

[post("change_rough_prefered_length")]
async fn change_rough_preferred_length(
    req_body: Json<ChangeRoughPreferredLengthRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ChangeRoughPreferedLengthRequestStruct { preferred_lengths, password } = req_body.into_inner();
    let mut res_body: ChangeRoughPreferedLengthResponseStruct = ChangeRoughPreferedLengthResponseStruct::new();

    // Authenticate, is this done outside of this function?
    // Validate preferred lengths else error
    let validated_preferred_lengths = validate_preferred_lengths(&preferred_lengths);
    if validated_preferred_lengths.is_err() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_username.err().unwrap()),
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
        update_rough_preferred_lengths_for_user_in_pg_content_delivery_table(&pool, &user).await;

    // if sql update error then return an error
    if update_result.is_err() {
        res_body.account_error = AccountError {
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


fn validate_preferred_lengths(preferred_lengths: &PreferredLengths) -> {
    // Ensure that no value is repeated in ChangeRoughPreferedLengthStruct if not none
    // If none is one of the values then all subsequent must be none
}
     
