from fastapi import Request, status
from src.services import stripe_service, paypal_service
from src.datatypes.payment_method import PaymentMethodEnum
from src.datatypes.subscription_method import SubscriptionMethodEnum

async def retry_failed_payment_view(
    request: Request,
    payment_type: PaymentTypeEnum,
    payment_method: PaymentMethodEnum|SubscriptionMethodEnum,
    payment_id: str,
    customer_id_token: str | None,
):
    user_uuid = request.state.user_uuid

    if type(payment_method) == PaymentMethodEnum:
        if type(payment_method) == PaymentMethodEnum.Stripe:
            success, message = stripe_service.retry_stripe_payment(payment_id)
        if type(payment_method) == PaymentMethodEnum.Paypal:
            success, message = paypal_service.retry_paypal_payment(payment_id)
        if type(payment_method) == PaymentMethodEnum.Crypto:
            pass

    elif type(payment_method) == SubscriptionMethodEnum:
        if customer_id_token == None:
            raise HTTPException(status_code=400, detail="customer id token is required")

        customer_id = cache_service.get_customer_id_from_token(customer_id_token)
        if validate_uuid_and_customer_id(user_uuid, customer_id) == false:
            raise HTTPException(status_code=400, detail="Invalid customer id")

        if type(payment_method) == SubscriptionMethodEnum.Stripe:
            success, message = stripe_service.retry_stripe_subscription_payment(customer_id, payment_id)
        if type(payment_method) == SubscriptionMethodEnum.Paypal:
            success, message = paypal_service.retry_paypal_subscription_payment(customer_id, payment_id)

    else:
        raise HTTPException(status_code=400, detail="Invalid provider")

    if not success:
        raise HTTPException(status_code=400, detail=message)

    return {"message": "Payment retried successfully"}