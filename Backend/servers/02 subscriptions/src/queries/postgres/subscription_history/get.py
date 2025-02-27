from sqlalchemy import text

def from_user_uuid(user_uuid: str, limit):
    query = text("""
    SELECT ()
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

        return DiscountClass(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")