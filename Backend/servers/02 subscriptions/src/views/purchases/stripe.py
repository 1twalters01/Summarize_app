from fastapi import APIRouter, HTTPException, status
from src.middleware.authentication import is_authenticated
from src.services.stripe_service import create_stripe_checkout_session

router = APIRouter()

# Change it from price to an enum
async def stripe_one_time_payment(request: Request, price: float):
    """Initiate a PayPal one-time payment."""

    user_uuid = request.state.user_uuid
    response = create_stripe_purchase_checkout_session(user_uuid, price)

    if "error" in response:
        raise HTTPException(
            status_code=400,
            detail=response["error"]
        )

    # save in redis user_uuid:purchase_enum

    return response