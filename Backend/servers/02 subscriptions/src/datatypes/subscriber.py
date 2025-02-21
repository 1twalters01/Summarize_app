from src.datatypes.payment_method import PaymentMethodEnum

class Subscriber():
    id: int
    user_id: int
    customer_id: str|None
    payment_method: PaymentMethodEnum