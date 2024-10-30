from fastapi import Depends, Request, status
from fastapi.responses import JSONResponse
from sqlalchemy import create_engine, text
from sqlalchemy.orm import sessionmaker, Session
from dotenv import load_dotenv
from ...utils.encryption import encrypt
import jwt
import os

load_dotenv()


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


async def get_subscription_status(
    user_uuid: str, db: Session = Depends(get_pg_db)
) -> bool | None:
    # get is_subscribed in users from user_uuid
    query = "SELECT s.is_subscribed FROM subscribers s JOIN users ON s.userID=u.userID WHERE u.userID=:id"
    is_subscribed = db.execute(text(query), {"id": user_uuid}).fetchone()
    db.close()
    return is_subscribed[0] if is_subscribed else None


def validate_stripe_customer_id(stripe_customer_id):
    if stripe_customer_id.len() < 5:
        return False
    return True


async def create_stripe_customer(request: Request, stripe_customer_id: str):
    headers = request.headers
    bearer: str | None = headers.get("bearer_token")
    match bearer:
        case None:
            response = {"error", "no token"}
            return JSONResponse(
                content=response, status_code=status.HTTP_400_BAD_REQUEST
            )
        case bearer:
            encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])

    # get claims from token
    print(decoded_jwt)
    user_uuid = decoded_jwt["sub"]

    # get user_uuid
    is_subscribed: bool | None = await get_subscription_status(user_uuid)
    if is_subscribed == None:
        response = {"error", "Invalid token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    if is_subscribed == True:
        response = {"error": "Customer already exists"}
        return JSONResponse(content=response, status_code=status.HTTP_409_CONFLICT)

    # Validate stripe_customer_ID
    if validate_stripe_customer_id(stripe_customer_id) == False:
        response = {"error": "Invalid stripe customer ID"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    # set new customer
    try:
        encrypted_customer_id = encrypt(stripe_customer_id)
        db = get_pg_db()
        if db == None:
            response = {"error": "Server error"}
            return JSONResponse(
                content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
            )
        # SET subscription_id=NULL, customer_id=encrypt(customer_id), m.method="Stripe" INTO subscriptions where user_uuid=user_uuid (joins, fix this)
        query = 'SET s.customer_id=:encrypted_customer_id, s.subscription_id=NULL, m.method="Stripe" INTO subscriptions WHERE user_uuid=:user_uuid'
        db.execute(
            text(query),
            {"encrypted_customer_id": encrypted_customer_id, "user_uuid": user_uuid},
        ).fetchone()
        response = {"success": True}
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)
    except:
        response = {"success": False}
        return JSONResponse(
            content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
        )
