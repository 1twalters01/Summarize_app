from sqlalchemy import text

def subscription_method_and_user_id_from_subscriber_id(subscriber_id):
    query = text("""
    SELECT (
        user_id,
        subscription_method_enum,
    )
    FROM subscribers
    WHERE id = subscriber_id
    """)
    
    try:
        result = db.execute(
            query,
            {
                "user_uuid": user_uuid,
            },
        ).fetchone()
        db.close()

        return DiscountClass(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")