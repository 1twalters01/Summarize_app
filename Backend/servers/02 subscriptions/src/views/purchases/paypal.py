from fastapi import APIRouter, HTTPException, status
from src.middleware.authentication import is_authenticated
from src.services.paypal_service import create_paypal_order

router = APIRouter()

# Change it from price to an enum
async def paypal_one_time_payment(request: Request, price: float):
    """Initiate a PayPal one-time payment."""

    user_uuid = request.state.user_uuid
    response = create_paypal_purchase_order(user_uuid, price)

    if "error" in response:
        raise HTTPException(
            status_code=400,
            detail=response["error"]
        )

    # save in redis user_uuid:purchase_enum

    return response