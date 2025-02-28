from sqlalchemy import text

def from_user_uuid(user_uuid: str, limit):
    query = text("""
    SELECT (
        id,
        user_id,
        payment_id,
        payment_method_enum,
        payment_tier_enum,
        payment_date,
        duration,
        payment_start_date
    )
    FROM payment_history ph
    JOIN users u on ph.user_id = u.id
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
