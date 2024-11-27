from sqlalchemy import Engine, text

def get_admin_status(engine: Engine, user_uuid: str):
    with engine.connect() as conn:
        query = text("SELECT is_admin FROM users WHERE uuid = :uuid")
        result = conn.execute(query, {"uuid": user_uuid}).fetchone()
        if result == None:
            return None
        else:
            return bool(result)

