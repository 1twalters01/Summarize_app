from sqlalchemy import text

def increment_current_uses_from_code(discount_code: str):
    query = text("""
        UPDATE discount_codes
        SET current_uses = current_uses + 1
        WHERE discount_code = :discount_code
    """)

    try:
        db.execute(
            query,
            {
                "discount_code": discount_code,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")