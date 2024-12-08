from fastapi import Depends
from sqlalchemy import text
from sqlalchemy.orm import Session
from src.models.subscriber import Subscriber
from src.utils.database_connections import get_pg_db


def get_subscriber(
    user_uuid: str, db: Session = Depends(get_pg_db)
) -> Subscriber | None:
    # get subscriptions fields from user_uuid
    query = "SELECT s.is_subscribed, s.has_trial FROM subscribers s JOIN users ON s.userID=u.userID WHERE u.userID=:id"
    subscriber = db.execute(text(query), {"id": user_uuid}).fetchall()
    if subscriber[0] == None or subscriber[1] == None:
        return None
    subscriber = Subscriber(is_subscribed=subscriber[0], has_trial=subscriber[1])
    db.close()
    return subscriber


def get_or_create_subscriber(
    user_uuid: str, db: Session = Depends(get_pg_db)
) -> Subscriber | None:
    # make this actually create or get not just get
    # get subscriptions fields from user_uuid
    query = "SELECT s.is_subscribed, s.has_trial FROM subscribers s JOIN users ON s.userID=u.userID WHERE u.userID=:id"
    subscriber = db.execute(text(query), {"id": user_uuid}).fetchall()
    if subscriber[0] == None or subscriber[1] == None:
        return None
    subscriber = Subscriber(is_subscribed=subscriber[0], has_trial=subscriber[1])
    db.close()
    return subscriber


async def get_detailed_subscriber(
    user_uuid: str, db: Session = Depends(get_pg_db)
) -> Subscriber | None:
    # get is_subscribed in users from user_uuid
    query = "SELECT s.is_subscribed, s.has_trial, s.end_date, s.start_date, s.subscription_id, s.customer_id FROM subscribers s JOIN users ON s.userID=u.userID WHERE u.userID=:id"
    result = db.execute(text(query), {"id": user_uuid}).fetchall()
    if (
        result[0] == None
        or result[1] == None
        or result[2] == None
        or result[3] == None
        or result[4] == None
        or result[5] == None
    ):
        return None
    subscriber = Subscriber(
        is_subscribed=result[0],
        has_trial=result[1],
        s_date=result[2],
        e_date=result[3],
        s_id=result[4],
        c_id=result[5],
    )
    db.close()
    return subscriber


def get_subscription_trial_status(
    user_uuid: str, db: Session = Depends(get_pg_db)
) -> bool | None:
    """get subscription trial status in subscriptions from user_uuid"""
    query = text(
        "SELECT s.has_trial FROM subscribers s JOIN users ON s.userID=u.userID WHERE u.userID=:id"
    )
    has_trial = db.execute(query, {"id": user_uuid}).fetchone()
    db.close()
    return has_trial[0] if has_trial else None


def get_subscription_status(
    user_uuid: str, db: Session = Depends(get_pg_db)
) -> bool | None:
    """get is_subscribed in subscriptions from user_uuid"""
    query = text(
        "SELECT s.is_subscribed FROM subscribers s JOIN users ON s.userID=u.userID WHERE u.userID=:id"
    )
    is_subscribed = db.execute(query, {"id": user_uuid}).fetchone()
    db.close()
    return is_subscribed[0] if is_subscribed else None
