from fastapi import Request, status

class payment_types(Enum):
    Subscription_Monthly = 1
    Subscription_Yearly = 2
    Payment_1_Month = 3
    Payment_3_Months = 3
    Payment_1_Year = 3

class payment_providers(Enum):
    Stripe = 1
    Paypal = 2
    Crypto = 3

def create_checkout_session_view(request: Request, payment_provider: payment_providers, payment_type: payment_types):
    if payment_type == payment_types.Subscription_Monthly | payment_types.Subscription_Yearly:
        raise HTTPException(status_code=400, detail="Invalid payment type")

    # See if discount code code is selected for user
    user_uuid = request.state.user_uuid
    code = cache_service.get_discount_code_from_user_uuid(user_uuid)

    if payment_provider == payment_providers.Stripe:
        response = create_stripe_purchase_checkout_session(user_uuid, payment_types, code)
    elif payment_provider == payment_providers.Paypal:
        response = create_paypal_purchase_order(user_uuid, payment_types, code)
    elif payment_provider == payment_providers.Crypto:
        response = create_crypto_purchase_order(user_uuid, payment_types, code)
    else:
        raise HTTPException(
            status_code=400,
            detail="Invalid payment provider"
        )

    if "error" in response:
        raise HTTPException(
            status_code=400,
            detail=response["error"]
        )

    # save in redis user_uuid:payment_types

    return response