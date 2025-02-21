from datetime import datetime
from src.datatypes.payment_type import PaymentTypeEnum

class Subscription():
    id: int
    subscriber_id: int
    subscription_id: str
    payment_tier_enum: PaymentTypeEnum
    subscription_start_date: datetime
    subscription_end_date: datetime|None
    cancellation_date: datetime|None