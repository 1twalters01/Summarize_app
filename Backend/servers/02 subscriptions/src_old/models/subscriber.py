from datetime import datetime


class PaymentMethod:
    id: int
    payment_method: str


class Subscriber:
    is_subscribed: bool
    has_trial: bool
    customer_id: str | None
    subscription_id: str | None
    payment_method: PaymentMethod | None
    start_date: datetime | None
    end_date: datetime | None

    def __init__(
        self,
        is_subscribed,
        has_trial,
        c_id=None,
        s_id=None,
        method=None,
        s_date=None,
        e_date=None,
    ):
        self.is_subscribed = is_subscribed
        self.has_trial = has_trial
        self.customer_id = c_id
        self.subscription_id = s_id
        self.payment_method = method
        self.start_date = s_date
        self.end_date = e_date
