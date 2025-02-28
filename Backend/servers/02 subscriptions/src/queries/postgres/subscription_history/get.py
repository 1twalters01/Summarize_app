from sqlalchemy import text

def from_user_uuid(user_uuid: str, limit):
    query = text("""
    SELECT (
        id,
        subscriber_id,
        encrypted_subscription_id,
        payment_tier_enum,
        duration,
        subscription_start_date,
        subscription_end_date,
        cancellation_date,
    )
    FROM subscription_history sh
    JOIN subscribers s on sh.subscriber_id = s.id
    JOIN users u on s.user_id = u.id
    WHERE u.uuid = :user_uuid
    LIMIT :limit
    """)

    try:
        result = db.execute(
            query,
            {
                "user_uuid": user_uuid,
                "limit": limit,
            },
        ).fetchone()
        db.close()

        return SubscriptionHistory(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")

def from_encrypted_subscription_id(encrypted_subscription_id_str: str):
    query = text("""
    SELECT (
        id,
        subscriber_id,
        encrypted_subscription_id,
        payment_tier_enum,
        duration,
        subscription_start_date,
        subscription_end_date,
        cancellation_date,
    )
    FROM subscription_history ph
    WHERE encrypted_subscription_id = :encrypted_subscription_id_str
    """)

    try:
        result = db.execute(
            query,
            {
                "encrypted_subscription_id_str": encrypted_subscription_id_str,
            },
        ).fetchone()
        db.close()

        return SubscriptionHistory(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")