from fastapi import Request, status
from enum import Enum
from src.datatypes.retry import Retry

async def retry_failed_payment_view(request: Request, data: Retry):
    user_uuid = request.state.user_uuid

    if data.customer_id_token == None:
        customer_id: str|None = user_service.get_customer_id_from_uuid_and_payment_provider(user_uuid, data.payment_provider)
    else:
        customer_id = cache_service.get_customer_id_from_token(data.customer_id_token)
        if validate_uuid_and_customer_id(user_uuid, customer_id) == false:
            raise HTTPException(status_code=400, detail="Invalid customer id tokke")

    # customer id may be equal to None if it is a one time payment
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