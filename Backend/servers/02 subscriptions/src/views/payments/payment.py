from fastapi import Request, status
from src.datatypes.payment_providers import PaymentProviderEnum
from src.datatypes.payment_type import PaymentTypeEnum

def create_checkout_session_view(request: Request, payment_provider: PaymentProviderEnum, payment_type: PaymentTypeEnum):
    if payment_type == PaymentTypeEnum.Subscription_Monthly | PaymentTypeEnum.Subscription_Yearly:
        raise HTTPException(status_code=400, detail="Invalid payment type")

    # See if discount code code is selected for user
    user_uuid = request.state.user_uuid
    code = cache_service.get_discount_code_from_user_uuid(user_uuid)

    if payment_provider == PaymentProviderEnum.Stripe:
        response = create_stripe_purchase_checkout_session(user_uuid, payment_type, code)
    elif payment_provider == PaymentProviderEnum.Paypal:
        response = create_paypal_purchase_order(user_uuid, payment_type, code)
    elif payment_provider == PaymentProviderEnum.Crypto:
        response = create_crypto_purchase_order(user_uuid, payment_type, code)
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

    # save in redis user_uuid:payment_type

    return response