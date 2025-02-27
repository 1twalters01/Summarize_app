from sqlalchemy import text

def from_history_item(
    payment_history_id: int|None = None,
    subscription_history_id: int|None = None
):
    if (payment_history_id == None) == (subscription_history_id == None):
        return "One of payment history id and subscription history id must be None and the other must be an id"
    
    now = datetime.now()

    if payment_history_id != None:
        query = text("""
            INSERT INTO refund_requests (user_id, payment_history_id, subscription_history_id, refund_date)
            SELECT u.id, ph.id, :subscription_history_id, :now
            FROM users u
            JOIN payment_history ph on ph.user_id = u.id
            WHERE ph.payment_id = :payment_history_id
        """)
    else:
        query = text("""
            INSERT INTO refund_requests (user_id, payment_history_id, subscription_history_id, refund_date)
            SELECT u.id, :payment_history_id, sh.id, :now
            FROM users u
            JOin subscribers s on s.user_id = u.id
            JOIN subscription_history sh on sh.subscriber_id = s.id
            WHERE sh.subscription_id = :subscription_history_id
        """)

    try:
        db.execute(
            query,
            {
                "payment_history_id": payment_history_id,
                "subscription_history_id": subscription_history_id,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")