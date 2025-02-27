from sqlalchemy import text

def from_user_uuid(user_uuid: str):
    query = text("""
        INSERT INTO subscription_metadata (user_id)
        SELECT u.id
        WHERE u.uuid = user_uuid
    """)

    try:
        db.execute(
            query,
            {
                "user_uuid": user_uuid,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")