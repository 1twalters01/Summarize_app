from src.datatypes.payment_type import PaymentTypeEnum
from src.datatypes.payment_tier import PaymentTierEnum

class Price():
    id: int
    payment_tier_enum: PaymentTierEnum
    payment_type_enum: PaymentTypeEnum
    price_gbp: float