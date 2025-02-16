from fastapi import Request, status
from datetime import datetime
from enum import Enum

class discount_class():
    id: int
    code: str
    max_uses: int|None
    current_uses: int|None
    created_at: datetime
    expires_at: datetime|None


class payment_type(Enum):
    Subscription_Monthly = 1
    Subscription_Yearly = 2
    Payment_1_Month = 3
    Payment_3_Months = 3
    Payment_1_Year = 3

class tier(Enum):
    Premium = 1

async def apply_discount_view(request: Request, discount_code: str, payment_type: payment_type, tier: tier):
    discount: discount_class = discount_service.get_discount_from_code(discount_code)
    
    if not discount:
        raise HTTPException(status_code=400, detail="Invalid discount code")

    if discount.expires_at and discount.expires_at < datetime.utcnow():
        raise HTTPException(status_code=400, detail="Discount code expired")

    if discount.max_uses and discount.current_uses >= discount.max_uses:
        raise HTTPException(status_code=400, detail="Discount code has reached its usage limit")

    if discount_service.validate_code_and_payment_type(discount_code, payment_type) == False:
        raise HTTPException(status_code=400, detail="Discount code is not valid for the given payment type")

    user_uuid = request.state.user_uuid
    # cache key: f"discount for {user_uuid}", value: code

    return {
        "discount_value": discount.discount_value,
        "discount_type": discount.discount_type,
    }