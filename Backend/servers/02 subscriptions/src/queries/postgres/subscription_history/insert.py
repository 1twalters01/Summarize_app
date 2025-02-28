from sqlalchemy import text
from datetime.datetime import now

def from_customer_id_and_encrypted_subscription_id(customer_id_str, encrypted_subscription_id):
    subscription_start_date = now()
    query = text("""
        INSERT INTO subscription_history (subscriber_id, encrypted_subscription_id, subscription_start_date)
        SELECT s.id, :encrypted_subscription_id, :subscription_start_date
        FROM subscribers s
        where s.customer_id = :customer_id_str
    """)
    try:
        db.execute(
            query,
            {
                "customer_id_str": customer_id_str,
                "encrypted_subscription_id": encrypted_subscription_id,
                "subscription_start_date": subscription_start_date,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")