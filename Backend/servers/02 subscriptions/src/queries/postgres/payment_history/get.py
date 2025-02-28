from sqlalchemy import text

def from_user_uuid(user_uuid: str, limit: int|None):
    query = text("""
    SELECT (
        id,
        user_id,
        encrypted_payment_id,
        payment_method_enum,
        payment_tier_enum,
        duration,
        payment_start_date
        payment_end_date,
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

        return PaymentHistory(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")

def from_encrypted_payment_id(encrypted_payment_id_str: str):
    query = text("""
    SELECT (
        id,
        user_id,
        encrypted_payment_id,
        payment_method_enum,
        payment_tier_enum,
        duration,
        payment_start_date
        payment_end_date,
    )
    FROM payment_history ph
    WHERE encrypted_payment_id = :encrypted_payment_id_str
    """)

    try:
        result = db.execute(
            query,
            {
                "encrypted_payment_id_str": encrypted_payment_id_str,
            },
        ).fetchone()
        db.close()

        return PaymentHistory(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")