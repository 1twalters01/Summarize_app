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

        return Subscriber(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")

def subscribers_from_user_uuid(user_uuid):
    query = text("""
    SELECT (
        id,
        user_id,
        encrypted_customer_id,
        subscription_method_enum,
    )
    FROM subscribers s
    JOIN users u
    ON s.user_id = u.id
    WHERE u.uuid = :user_uuid
    """)
    
    try:
        results = db.execute(
            query,
            {
                "user_uuid": user_uuid,
            },
        ).fetchone()
        db.close()

        return [Subscriber(*result) for result in results] if results else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")