from fastapi import Request, status
from enum import Enum

class payment_providers(Enum):
    Stripe = 1
    Paypal = 2
    Crypto = 3

class retry_class():
    payment_provider: payment_providers
    customer_id_token: str|None

async def retry_failed_payment_view(request: Request, data: retry_class):
    if data.customer_id_token == None:
        user_uuid = request.state.user_uuid
        customer_id = user_service.get_customer_id_from_uuid_and_payment_provider(user_uuid, data.payment_provider)
    else:
        customer_id = cache_service.get_customer_id_from_token(data.customer_id)

    payment_provider = data.payment_provider
    if payment_provider == "stripe":
        success, message = retry_stripe_payment(customer_id)
    elif payment_provider == "paypal":
        success, message = retry_paypal_payment(customer_id)
    else:
        raise HTTPException(status_code=400, detail="Invalid payment provider")

    if not success:
        raise HTTPException(status_code=400, detail=message)

    return {"message": "Payment retried successfully"}