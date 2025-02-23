from src.datatypes.payment_type import PaymentTypeEnum
from src.datatypes.discount_class import DiscountClass
from sqlalchemy import text

def from_code_and_payment_type_enum(discount_code: str, payment_type_enum: PaymentTypeEnum) -> DiscountClass|None:
    query = text("""
        SELECT * from discount_codes
        WHERE discount_code = :discount_code AND payment_type_enum = :payment_type_enum
    """)

    try:
        result = db.execute(
            query,
            {
                "discount_code": discount_code,
                "payment_type_enum": payment_type_enum
            },
        ).fetchone()
        db.close()

        return DiscountClass(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")

def get_discounts_by_code(discount_code: str) -> list[DiscountClass]|None:
    query = text("""
        SELECT * FROM discount_codes
        WHERE discount_code = :discount_code
    """)

    try:
        results = db.execute(
            query,
            {
                "discount_code": discount_code
            }
        ).fetchall()
        db.close()

        return [DiscountClass(*result) for result in results] if results else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")