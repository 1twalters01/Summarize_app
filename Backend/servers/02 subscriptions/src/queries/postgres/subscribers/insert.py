from src.datatypes.payment_method import PaymentMethodEnum
from sqlalchemy import text

def from_user_uuid_and_customer_id_and_payment_method(user_uuid: str, customer_id: str, payment_method: PaymentMethodEnum):
    query = text(""""
        INSERT INTO subscribers (user_id, customer_id, payment_method)
        SELECT u.id, :customer_id, :payment_method
        FROM users u
        WHERE u.uuid = user_uuid
    """)

    try:
        db.execute(
            query,
            {
                "user_uuid": user_uuid,
                "customer_id": customer_id,
                "payment_method": payment_method,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")

def from_user_uuid_and_payment_method(user_uuid: str, payment_method: PaymentMethodEnum):
    query = text(""""
        INSERT INTO subscribers (user_id, payment_method)
        SELECT u.id, :payment_method
        FROM users u
        WHERE u.uuid = user_uuid
    """)

    try:
        db.execute(
            query,
            {
                "user_uuid": user_uuid,
                "payment_method": payment_method,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")