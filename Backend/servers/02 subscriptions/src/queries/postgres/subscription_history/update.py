from datetime.datetime import duration, datetime
from sqlalchemy import text

def duration_and_end_date_from_encrypted_subscription_id(
    duration: duration|None,
    subscription_end_date: datetime|None,
    encrypted_subscription_id: str,
):
    query = text("""
        UPDATE subscription_history
        SET duration = :duration,
            subscription_end_date = :subscription_end_date
        WHERE encrypted_subscription_id = :encrypted_subscription_id
    """)

    try:
        db.execute(
            query,
            {
                "duration": duration,
                "subscription_end_date": subscription_end_date,
                "encrypted_subscription_id": encrypted_subscription_id,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")