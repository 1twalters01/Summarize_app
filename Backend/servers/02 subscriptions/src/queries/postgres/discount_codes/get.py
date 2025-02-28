from src.datatypes.payment_type import PaymentTypeEnum
from src.datatypes.discount_class import DiscountClass
from sqlalchemy import text

# This query should be in discount_payment_types
def from_discount_code(discount_code: str) -> DiscountClass|None:
    query = text("""
        SELECT (
            id,
            discount_code,
            max_uses,
            current_uses,
            created_at,
            expires_at,
            discount_value,
            discount_type_enum
        )
        from discount_codes
        WHERE discount_code = :discount_code
    """)

    try:
        result = db.execute(
            query,
            {
                "discount_code": discount_code,
            },
        ).fetchone()
        db.close()

        return DiscountClass(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")
