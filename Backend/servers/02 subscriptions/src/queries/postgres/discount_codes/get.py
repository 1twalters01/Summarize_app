from src.datatypes.payment_type import PaymentTypeEnum
from src.datatypes.discount_class import DiscountClass
from sqlalchemy import text

class DiscountClass():
    id: int
    code: str
    max_uses: int|None
    current_uses: int|None
    created_at: datetime
    expires_at: datetime|None

def from_code_and_payment_type_enum(discount_code: str, payment_type_enum: PaymentTypeEnum):
    query = text("""
        SELECT * from discount_codes
        WHERE discount_code = :discount_code AND payment_type_enum = :payment_type_enum
    """)

    result = db.execute(
        query,
        {
            "discount_code": discount_code,
            "payment_type_enum": payment_type_enum
        },
    ).fetchone()
    db.close()

    if result:
        return DiscountClass(*result)
    return None