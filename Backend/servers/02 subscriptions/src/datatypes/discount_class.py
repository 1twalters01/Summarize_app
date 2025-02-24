from src.datatypes.payment_type import PaymentTypeEnum

class DiscountClass():
    id: int
    code: str
    max_uses: int|None
    current_uses: int|None
    created_at: datetime
    expires_at: datetime|None

    def __init__(self, id: int, code: str, max_uses: int | None, current_uses: int | None, created_at: datetime, expires_at: datetime | None):
        self.id = id
        self.code = code
        self.max_uses = max_uses
        self.current_uses = current_uses
        self.created_at = created_at
        self.expires_at = expires_at

class DiscountPayment():
    id: int
    discount_code_id: int
    payment_type: PaymentTypeEnum

    def __init__(self, id, discount_code_id: int, payment_type: PaymentTypeEnum):
        self.id = id
        self.discount_code_id = discount_code_id
        self.payment_type = payment_type