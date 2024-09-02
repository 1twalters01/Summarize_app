from fastapi import Depends, Request, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel
from sqlalchemy import create_engine, text
from sqlalchemy.orm import sessionmaker, Session
from dotenv import load_dotenv
import stripe
import jwt
import os

load_dotenv()


class Subscription(BaseModel):
    success_url: str
    cancel_url: str


class Subscriber:
    trial_status: bool
    is_subscribed: bool

    def __init__(self, is_subscribed, trial_status):
        self.is_subscribed = is_subscribed
        self.trial_status = trial_status


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


async def get_subscriber(
    user_uuid: str, db: Session = Depends(get_pg_db)
) -> Subscriber | None:
    # get is_subscribed in users from user_uuid
    query = "SELECT s.is_subscribed, s.trial FROM subscribers s JOIN users ON s.userID=u.userID WHERE u.userID=:id"
    result = db.execute(text(query), {"id": user_uuid}).fetchall()
    if result[0] == None or result[1] == None:
        return None
    subscriber = Subscriber(is_subscribed=result[0], trial_status=result[1])
    db.close()
    return subscriber


async def retrieve_status(request: Request, subscription: Subscription):
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

    subscriber: Subscriber | None = await get_subscriber(user_uuid)
    if subscriber == None:
        response = {"error", "Invalid token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    success_url = subscription.success_url
    cancel_url = subscription.cancel_url

    if subscriber.is_subscribed == False:
        stripe.api_key = os.getenv("STRIPE_SECRET_KEY")
        customer = stripe.Customer.create()
        stripe_checkout = build_stripe_checkout(
            subscriber, customer, success_url, cancel_url
        )
        stripe_url = stripe_checkout.url

        response = {
            "subscribed": subscriber.is_subscribed,
            "trial": subscriber.trial_status,
            "stripe_customer_id": customer.id,
            "stripe_url": stripe_url,
        }
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)
    else:
        response = {
            "subscribed": subscriber.is_subscribed,
            "trial": subscriber.trial_status,
        }
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)


def build_stripe_checkout(subscriber, customer, success_url, cancel_url):
    if not subscriber:
        error = "Subscriber does not exist)"
        raise Exception(error)

    prices = stripe.Price.list(
        lookup_keys=["Conjugat Premium"], expand=["data.product"]
    )

    line_items = [
        {
            "price": prices.data[0].id,
            "quantity": 1,
        },
    ]

    checkout_kwargs = {
        "line_items": line_items,
        "customer": customer,
        "mode": "subscription",
        "success_url": success_url,
        "cancel_url": cancel_url,
    }

    if subscriber.trial == True:
        checkout_kwargs["subscription_data"] = {"trial_period_days": 7}

    checkout_session = stripe.checkout.Session.create(**checkout_kwargs)
    return checkout_session

