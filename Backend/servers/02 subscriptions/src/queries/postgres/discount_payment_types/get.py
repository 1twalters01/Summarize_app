from src.datatypes.payment_type import PaymentTypeEnum
from src.datatypes.discount_class import DiscountClass
from sqlalchemy import text

def from_discount_code_and_payment_type(discount_code: str, payment_type_enum: PaymentTypeEnum) -> DiscountClass|None:
    query = text("""
        SELECT * from discount_codes
        WHERE discount_code = :discount_code AND payment_type_enum = :payment_type_enum
    """)

    try:
        result = db.execute(
            query,
            {
                "discount_code": discount_code,
                "payment_type_enum": payment_type_enum,
            },
        ).fetchone()
        db.close()

        return DiscountClass(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")
        
def from_discount_code(discount_code):
    def from_code(discount_code: str) -> list[DiscountClass]|None:
    query = text("""
        SELECT * FROM discount_codes
        WHERE discount_code = :discount_code
    """)

    try:
        results = db.execute(
            query,
            {
                "discount_code": discount_code,
            }
        ).fetchall()
        db.close()

        return [DiscountClass(*result) for result in results] if results else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")

def from_payment_type_enum(payment_type_enum: PaymentTypeEnum) -> list[DiscountClass]|None:
    query = text("""
        SELECT * FROM discount_codes
        WHERE payment_type_enum = :payment_type_enum
    """)

    try:
        results = db.execute(
            query,
            {
                "discount_code": discount_code,
            }
        ).fetchall()
        db.close()

        return [DiscountClass(*result) for result in results] if results else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")