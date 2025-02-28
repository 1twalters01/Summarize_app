from src.datatypes.payment_type import PaymentTypeEnum
from src.datatypes.discount_class import DiscountClass
from sqlalchemy import text

def from_discount_code_and_payment_type(discount_code: str, payment_type_enum: PaymentTypeEnum) -> DiscountClass|None:
    query = text("""
        SELECT (id, discount_code_id, payment_type_enum)
        FROM discount_payment_types dpt
        JOIN discount_codes dc
            ON dpt.discount_code_id = dc.id
        WHERE dpt.payment_type_enum = :payment_type_enum
            AND dc.discount_code = :discount_code
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

        return DiscountPayment(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")

def from_payment_type_enum(payment_type_enum: PaymentTypeEnum) -> list[DiscountClass]|None:
    query = text("""
        SELECT (id, discount_code_id, payment_type_enum)
        FROM discount_payment_types
        WHERE payment_type_enum = :payment_type_enum
    """)

    try:
        results = db.execute(
            query,
            {
                "payment_type_enum": payment_type_enum,
            }
        ).fetchall()
        db.close()

        return [DiscountPayment(*result) for result in results] if results else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")

def from_discount_code(discount_code) -> list[DiscountClass]|None:
    query = text("""
        SELECT (id, discount_code_id, payment_type_enum)
        FROM discount_payment_types dpt
        JOIN discount_codes dc ON dpt.discount_code_id = dc.id
        WHERE dpt.payment_type_enum = :payment_type_enum
    """)

    try:
        results = db.execute(
            query,
            {
                "discount_code": discount_code,
            }
        ).fetchall()
        db.close()

        return [DiscountPayment(*result) for result in results] if results else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")
