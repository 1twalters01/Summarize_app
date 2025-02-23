from sqlalchemy import text
from datetime import datetime

def from_user_uuid(
    user_uuid: str,
    discount_code_id: int,
    payment_history_id: int|None = None,
    subscription_history_id: int|None = None
):
    if (payment_history_id == None) == (subscription_history_id == None):
        return "One of payment history id and subscription history id must be None and the other must be an id"
    
    now = datetime.now()

    query = text("""
        INSERT INTO applied_discounts (user_id, discount_code_id, applied_at, payment_history_id, subscription_history_id)
        SELECT u.id, :discount_code_id, :applied_at, :payment_history_id, :subscription_history_id
        FROM users u
        WHERE u.uuid = :user_uuid
    """)

    try:
        db.execute(
            query,
            {
                "user_uuid": user_uuid,
                "discount_code_id": discount_code_id,
                "applied_at": now,
                "payment_history_id": payment_history_id,
                "subscription_history_id": subscription_history_id,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")