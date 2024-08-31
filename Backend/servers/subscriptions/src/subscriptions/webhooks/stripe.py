from datetime import datetime
from dateutil.relativedelta import relativedelta
from fastapi import Depends, status
from fastapi.responses import JSONResponse
from dotenv import load_dotenv
from pydantic import BaseModel
from sqlalchemy import create_engine, text
from sqlalchemy.orm import Session, sessionmaker
from ...utils.encryption import encrypt
import os

load_dotenv()


class StripeObject(BaseModel):
    id: str
    customer: str


class StripeData(BaseModel):
    object: StripeObject


class StripeEvent(BaseModel):
    data: StripeData
    type: str

def get_pg_db() -> Session | None:
    DATABASE_URL = os.getenv("PG_URL")
    if DATABASE_URL == None:
        return None
    try:
        engine = create_engine(DATABASE_URL)
        SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
        db = SessionLocal()
        return db
    except:
        return None

def get_uuid_from_customer_id(customer_id: str, db: Session = Depends(get_pg_db)):
    query = "SELECT u.uuid FROM users u JOIN subscribers s ON s.userID=u.userID WHERE customer_id=:encrypted_customer_id"
    result = db.execute(text(query), {"encrypted_costomer_id": encrypt(customer_id)}).fetchone()
    if result == None:
        return None

    user_uuid: str = str(result)
    return user_uuid


async def stripe_webhook(event: StripeEvent):
    if event.type == "customer.deleted":
        customer_id = event.data.object.id
        user_uuid = get_uuid_from_customer_id(customer_id)
        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(
                content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
            )
        query = "UPDATE subscribers s SET s.subsciption_id=NULL, s.start_date=NULL, s.trial_status=FALSE, s.end_date=NULL, s.subscribed=FALSE, s.payment_method=NULL JOIN users u ON s.userID=u.userID WHERE u.user_uuid=:user_uuid"
        db = get_pg_db()
        if db == None:
            response = {"success": False}
            return JSONResponse(
                content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
            )
        db.execute(text(query), {"user_uuid": user_uuid})

        response = {"success": True}
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)

    elif event.type == "customer.subscription.trial_will_end":
        customer_id = event.data.object.customer
        user_uuid = get_uuid_from_customer_id(customer_id)
        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(
                content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
            )
        query = "SELECT email from users where uuid=user_uuid"
        db = get_pg_db()
        if db == None:
            response = {"success": False}
            return JSONResponse(
                content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
            )
        email = db.execute(text(query), {"user_uuid": user_uuid}).fetchone()
        if email == None:
            response = {"success": False}
            return JSONResponse(
                content=response, status_code=status.HTTP_404_NOT_FOUND
            )
        # Email user with email
        response = {"success": True}
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)

    elif event.type == "invoice.payment_succeeded":
        customer_id = event.data.object.customer
        user_uuid = get_uuid_from_customer_id(customer_id)
        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(
                content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
            )
        subscription_id = event.data.object.id
        start_date = datetime.now()
        query = "SELECT trial_status from users where uuid=user_uuid"
        db = get_pg_db()
        if db == None:
            response = {"success": False}
            return JSONResponse(
                content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
            )
        trial_status = db.execute(text(query), {"user_uuid": user_uuid}).fetchone()
        if trial_status == None:
            response = {"success": False}
            return JSONResponse(
                content=response, status_code=status.HTTP_404_NOT_FOUND
            )

        if trial_status == False:
            end_date = datetime.now() + relativedelta(months=1, days=2)
        else:
            end_date = datetime.now() + relativedelta(months=1, weeks=2, days=2)

        query = "UPDATE subscribers s SET s.customer_id=:customer_id, s.subscription_id=subscription_id, s.start_date=start_date, s.trial_status=False, s.end_date=end_date, s.subscribed=true, payment_method=stripe JOIN users u ON u.userID=s.userID WHERE u.uuid=user_uuid"
        db.execute(
            text(query),
            {"customer_id": customer_id,
             "subscription_id": subscription_id,
             "start_date":start_date,
             "end_date": end_date,
            }
        )
        response = {"success": True}
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)

    else:
        response = {"success": False}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)
