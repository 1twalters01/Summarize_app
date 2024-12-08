from fastapi import Depends
from sqlalchemy import text
from sqlalchemy.orm import Session
from src.utils.database_connections import get_pg_db


def get_admin_status(user_uuid: str, db: Session = Depends(get_pg_db)) -> bool | None:
    query = text("SELECT is_admin FROM users WHERE uuid = :uuid")
    admin_status = db.execute(query, {"uuid": user_uuid}).fetchone()
    db.close()
    if admin_status == None:
        return None
    else:
        return admin_status[0] if admin_status else None
