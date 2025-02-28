from sqlalchemy import text
from datetime import datetime

def from_user_uuid_and_discount_code_and_id_str(
    user_uuid: str,
    discount_code: str,
    payment_id_str: str|None = None,
    subscription_id_str: str|None = None,
):
    if (payment_id_str == None) == (subscription_id_str == None):
        return "One of payment history id and subscription history id must be None and the other must be an id"
    
    now = datetime.now()

    query = text("""
        INSERT INTO applied_discounts (user_id, discount_code, applied_at, payment_history_id, subscription_history_id)
        SELECT u.id, dc.id, :applied_at, ph.id, sh.id
        FROM users u, discount_codes dc, payment_history ph, subscription_history sh
        WHERE u.uuid = :user_uuid
            AND dc.discount_code = :discount_code
            AND ph.payment_id = :payment_id_str
            AND sh.subscription_id = :subscription_id_str
    """)

    try:
        db.execute(
            query,
            {
                "user_uuid": user_uuid,
                "discount_code": discount_code,
                "payment_id_str": payment_id_str,
                "subscription_id_str": subscription_id_str,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")