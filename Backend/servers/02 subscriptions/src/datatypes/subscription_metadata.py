from datetime import datetime
from src.datatypes.payment_type import PaymentTypeEnum

class SubscriptionMetadata():
    user_id: int
    payment_tier_enum: PaymentTypeEnum
    subscription_history_id: Int|None
    payment_history_id: Int|None
    has_trial: bool
    trial_start_date: datetime|None
    trial_end_date: datetime|None
