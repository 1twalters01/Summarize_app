from datetime.datetime import duration, datetime
from sqlalchemy import text

def duration_and_end_date_from_encrypted_payment_id(
    duration: duration|None,
    payment_end_date: datetime|None,
    encrypted_payment_id: str,
):
    query = text("""
        UPDATE payment_history
        SET duration = :duration,
            payment_end_date = :payment_end_date
        WHERE encrypted_payment_id = :encrypted_payment_id
    """)

    try:
        db.execute(
            query,
            {
                "duration": duration,
                "payment_end_date": payment_end_date,
                "encrypted_payment_id": encrypted_payment_id,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")