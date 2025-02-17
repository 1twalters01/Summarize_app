from enum import Enum

class PaymentTierEnum(Enum):
    none = 1
    premium = 2

class PaymentTypesEnum(Enum):
    Subscription_Monthly = 1
    Subscription_Yearly = 2
    Payment_1_Month = 3
    Payment_3_Months = 4
    Payment_1_Year = 5

class Price():
    id: int
    payment_tier_enum: PaymentTierEnum
    payment_type_enum: PaymentTypesEnum
    price_gbp: float

async def get_plans_view():
    prices: Price = discount_service.get_base_prices()
    return prices