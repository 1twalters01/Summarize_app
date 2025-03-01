from sqlalchemy import text
from src.datatypes.subscription_metadata import SubscriptionMetadata

def from_user_uuid(user_uuid: str):
    query = text("""
    SELECT ()
    FROM subscription_metadata sm
    JOIN users u on sm.user_id = u.id
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