from fastapi import Depends
from sqlalchemy import text
from sqlalchemy.exc import SQLAlchemyError
from sqlalchemy.orm import Session

from src.models.subscriber import Subscriber
from src.utils.database_connections import get_pg_db
from src.services.encryption_service import EncryptionService


def save_subscriber(
    user_uuid: str,
    subscriber: Subscriber,
    method: str,
    subscriber_id: str | None = None,
    customer_id: str | None = None,
    db: Session = Depends(get_pg_db),
):
    encryption_service = EncryptionService()
    encrypted_subscriber_id = encryption_service.encrypt(subscriber_id)
    encrypted_customer_id = encryption_service.encrypt(customer_id)
    method_id = method

    query = text(
        "INSERT INTO subscriptions s (trial_status, is_subscribed, end_date, method_id, subscription_id, customer_id, user_id) VALUES (:trial_status, :is_subscribed, :end_date, :method_id, :encrypted_subscription_id, :encrypted_customer_id, SELECT u.userID From users u Join subscriptions ON s.userID=u.userID WHERE u.user_uuid=:user_uuid) ON CONFLICT (user_id) UPDATE subscribers s SET s.trial_status=:trial_status, s.is_subscribed=:is_subscribed, s.end_date=:end_date, s.method_id=:method_id, s.subscription_id=:encrypted_subscription_id, s.userID=u.user_id JOIN users u on u.user_id=s.user_id WHERE u.user_uuid=user_uuid"
    )
    try:
        db.execute(
            query,
            {
                "trial_status": subscriber.has_trial,
                "is_subscribed": subscriber.is_subscribed,
                "encrypted_subscription_id": encrypted_subscriber_id,
                "encrypted_customer_id": encrypted_customer_id,
                "method_id": method_id,
                "user_uuid": user_uuid,
            },
        )
        db.commit()
    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")
    finally:
        db.close()
    # if error then return x else return y
