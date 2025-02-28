from sqlalchemy import text
from datetime.datetime import now

def from_customer_id_and_encrypted_payment_id(user_uuid, encrypted_payment_id):
    payment_start_date = now()
    query = text("""
        INSERT INTO payment_history (user_id, encrypted_payment_id, payment_start_date)
        SELECT s.id, :encrypted_payment_id, :payment_start_date
        FROM users u
        where u.uuid = :user_uuid
    """)
    try:
        db.execute(
            query,
            {
                "customer_id_str": customer_id_str,
                "encrypted_payment_id": encrypted_payment_id,
                "payment_start_date": payment_start_date,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")
        
def from_user_uuid(user_uuid):
    pass