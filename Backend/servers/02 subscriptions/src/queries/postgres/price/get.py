from sqlalchemy import text
from src.datatypes.price import Price

def all():
    query = text("""
        SELECT *
        FROM prices
    """)

    try:
        result = db.execute(
            query,
            {},
        ).fetchone()
        db.close()

        return Price(*result) if result else None
    except SQLAlchemyError as e:
        raise Exception(f"Failed to query db with error: {e}")