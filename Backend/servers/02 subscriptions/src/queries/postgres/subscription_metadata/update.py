from datetime import datetime
from sqlalchemy import text
from src.datatypes.payment_tier import PaymentTierEnum

def trial_metadata_from_user_uuid(
    user_uuid: str,
    has_trial: bool,
    trial_end_date: datetime|None,
):
    query = text("""
    UPDATE subscription_metadata
    SET has_trial = :has_trial
        trial_end_date = :trial_end_date
    WHERE user_uuid = :user_uuid
    """)

    try:
        db.execute(
            query,
            {
                "user_uuid": user_uuid,
                "has_trial": has_trial,
                "trial_end_date": trial_end_date,
            }
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")

def payment_metadata_from_user_uuid(
    user_uuid: str,
    payment_tier_enum: PaymentTierEnum,
    payment_id_str: int|None = None,
    subscription_id_str: int|None = None
):
    if (payment_id_str == None) == (subscription_id_str == None):
        return "One of payment history id and subscription history id must be None and the other must be an id"

    if payment_id_str != None:
        query = text("""
            UPDATE subscription_metadata
            SET payment_tier_enum = :payment_tier_enum
                payment_id_str = :payment_id_str
        """)
        dictionary = {
            "user_uuid": user_uuid,
            "payment_id_str": payment_id_str,
        }
    else:
        query = text("""
            UPDATE subscription_metadata
            SET payment_tier_enum = :payment_tier_enum
                subscription_id_str = :subscription_id_str
        """)
        dictionary = {
            "user_uuid": user_uuid,
            "subscription_id_str": subscription_id_str,
        }

    try:
        db.execute(
            query,
            dictionary
        )
        db.commit()
        db.close()
        return True

    except SQLAlchemyError as e:
        raise Exception(f"Failed to add to db with error: {e}")