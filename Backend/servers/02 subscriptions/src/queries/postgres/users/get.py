from sqlalchemy import text

def uuid_from_user_id(user_uuid:str):
    query = text("""
    SELECT (uuid)
    FROM users u
    WHERE u.uuid = user_uuid
    """)

    try:
        result = db.execute(
            query,
            {
                "user_uuid": user_uuid,
            },
        ).fetchone()
        db.close()

        return SubscriptionMetadata(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")