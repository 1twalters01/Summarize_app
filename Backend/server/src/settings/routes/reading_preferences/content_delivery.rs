use actix_web::{get, post, web::Json, HttpRequest, HttpResponse, Responder, Result};

pub enum PreferredLength {
    None,
    VShort, // < 5 min total read
    Short, // 5 <= x < 15 min total read 
    Medium, // 15 <= x 30 min total read
    Long, // 30 min <= 1 hour total read
    VLong, // > 1 hour total read
}

pub struct ChangeRoughPreferedLengthRequestStruct {
    preferred_lengths: PreferredLengths,
}

pub struct PreferredLengths {
    primary: Option<PreferredLength>,
    secondary: Option<PreferredLength>,
    tertiary: Option<PreferredLength>,
    quaternary: Option<PreferredLength>,
    quinternary: Option<PreferredLength>,
}

[post("change_rough_prefered_lengths")]
async fn change_rough_preferred_lengths(
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




pub enum DeliveryMethod {
    Text,
    Audio,
    Video,
}

pub struct ChangeDeliveryMethodRequestStruct {
    delivery_method: DeliveryMethods,
}

pub struct DeliveryMethods {
    primary: Option<DeliveryMethod>,
    secondary: Option<DeliveryMethodh>,
    tertiary: Option<DeliveryMethod>,
}
     
[post("change_delivery_methods")]
async fn change_delivery_methods(
    req_body: Json<ChangeDeliveryMethodRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ChangeDeliveryMethodRequestStruct { delivery_methods, password } = req_body.into_inner();
    let mut res_body: ChangeDeliveryMethodResponseStruct = ChangeDeliveryMethodResponseStruct::new();

    // Authenticate, is this done outside of this function?
    
    // Validate delivery methods else error
    let validated_delivery_methods = validate_delivery_methods(&preferred_lengths);
    if validated_delivery_methods.is_err() {
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
        update_delivery_methods_for_user_in_pg_content_delivery_table(&pool, &user).await;

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


fn validate_delivery_methods(delivery_methods: &DeliveryMethods) -> {
    // Ensure that no value is repeated in ChangeDeliveryMethodStruct if not none
    // If none is one of the values then all subsequent must be none
}

