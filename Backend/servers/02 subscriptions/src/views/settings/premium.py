from fastapi import Depends, Request, status
from fastapi.responses import JSONResponse
import jwt
from sqlalchemy import create_engine, text
from sqlalchemy.orm import sessionmaker, Session
from dotenv import load_dotenv
from datetime import datetime
from ...utils.encryption import decrypt, encrypt
from ...utils.encryption import decrypt
from ...services.paypal import show_sub_details, suspend_sub, activate_sub, cancel_sub
import stripe
import os

load_dotenv()


class Subscriber:
    trial_status: bool
    is_subscribed: bool
    end_date: datetime
    subscription_id: str|None
    customer_id: str|None

    def __init__(self, is_subscribed, trial_status, date, method=None, s_id=None, c_id=None):
        self.is_subscribed = is_subscribed
        self.trial_status = trial_status
        self.end_date = date
        self.method = method
        self.subscription_id = s_id
        self.customer_id = c_id


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


async def get_subscriber(
    user_uuid: str, db: Session = Depends(get_pg_db)
) -> Subscriber | None:
    # get is_subscribed in users from user_uuid
    query = "SELECT s.is_subscribed, s.trial, s.end_date, s.subscription_id, s.customer_id FROM subscribers s JOIN users ON s.userID=u.userID WHERE u.userID=:id"
    result = db.execute(text(query), {"id": user_uuid}).fetchall()
    if result[0] == None or result[1] == None:
        return None
    subscriber = Subscriber(
        is_subscribed=result[0],
        trial_status=result[1],
        date=result[2],
        s_id=result[3],
        c_id=result[4]
    )
    db.close()
    return subscriber


def obtain_method(subscriber):
    if subscriber:
        method = str(subscriber.method)
    else:
        method = None
    return method

def build_stripe_checkout(subscriber, customer, success_url, cancel_url):
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

    if not subscriber or subscriber.trial == True:
        checkout_kwargs["subscription_data"] = {"trial_period_days": 7}

    checkout_session = stripe.checkout.Session.create(**checkout_kwargs)
    return checkout_session

def save_subscriber(user_uuid: str, subscriber: Subscriber, method: str, subscriber_id: str|None=None, customer_id: str|None=None, db: Session=Depends(get_pg_db)):
    encrypted_subscriber_id = encrypt(subscriber_id)
    encrypted_customer_id = encrypt(customer_id)
    method_id = method
    
    query = "INSERT INTO subscriptions s (trial_status, is_subscribed, end_date, method_id, subscription_id, customer_id, user_id) VALUES (:trial_status, :is_subscribed, :end_date, :method_id, :encrypted_subscription_id, :encrypted_customer_id, SELECT u.userID From users u Join subscriptions ON s.userID=u.userID WHERE u.user_uuid=:user_uuid) ON CONFLICT (user_id) UPDATE subscribers s SET s.trial_status=:trial_status, s.is_subscribed=:is_subscribed, s.end_date=:end_date, s.method_id=:method_id, s.subscription_id=:encrypted_subscription_id, s.userID=u.user_id JOIN users u on u.user_id=s.user_id WHERE u.user_uuid=user_uuid"
    db.execute(
        text(query),
        {
            "trial_status": subscriber.trial_status,
            "is_subscribed": subscriber.is_subscribed,
            "encrypted_subscription_id": encrypted_subscriber_id,
            "encrypted_customer_id": encrypted_customer_id,
            "method_id": method_id,
            "user_uuid": user_uuid
        },
    )
    db.commit()
    # if error then return x else return y

def build_stripe_portal(stripe, subscriber, return_url):
    customer = decrypt(subscriber.customer_id)
    portalSession = stripe.billing_portal.Session.create(
        customer=customer,
        return_url=return_url,
    )
    return portalSession

async def get_premium_status(request: Request, data):
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

    stripe.api_key = os.getenv("STRIPE_SECRET_KEY")
    subscriber: Subscriber|None = await get_subscriber(user_uuid)
    if subscriber == None:
        response = {"error", "Invalid token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)
    method = obtain_method(subscriber)
    if subscriber.is_subscribed == False:
        if data["method"] == None:
            success_url = data["success_url"]
            cancel_url = data["cancel_url"]
            customer = stripe.Customer.create()
            stripe_url = build_stripe_checkout(
                subscriber, customer, success_url, cancel_url
            ).url

            response = {
                "subscribed": subscriber.is_subscribed,
                "trial": subscriber.trial_status,
                "stripe_customer_id": customer.id,
                "stripe_url": stripe_url,
            }
            return JSONResponse(content=response, status_code=status.HTTP_200_OK)

        if data["method"] == "Stripe":
            if subscriber.is_subscribed == False:
                customer_id = data["customer_id"]
                try:
                    save_subscriber(user_uuid, subscriber, method=data["method"], customer_id=customer_id)
                except:
                    error = {"error": "Stripe customer id was not found"}
                    return JSONResponse(content=error, status_code=status.HTTP_404_NOT_FOUND)
                response = {"success": "User created successfully"}
                return JSONResponse(content=response, status_code=status.HTTP_200_OK)

        if data["method"] == "Paypal":
            if subscriber.is_subscribed == False:
                subscriber_id = data.get("subscriber_id")
                try:
                    save_subscriber(user_uuid, subscriber, method=data["method"], subscriber_id=subscriber_id)
                except:
                    error = "Paypal customer id was not found"
                    return JSONResponse(content=error, status_code=status.HTTP_404_NOT_FOUND)
                response = "User created successfully"
                return JSONResponse(content=response, status_code=status.HTTP_200_OK)

    else:
        method = subscriber.method
        if subscriber.is_subscribed == True:
            if method == "Stripe":
                stripe.api_key = os.getenv("STRIPE_SECRET_KEY")
                return_url = data["return_url"]
                stripe_portal = build_stripe_portal(
                    stripe, subscriber, return_url
                )
                response = {
                    "method": method,
                    "subscribed": subscriber.is_subscribed,
                    "url": stripe_portal.url,
                }
                return JSONResponse(content=response, status_code=status.HTTP_200_OK)

            if method == "Paypal":
                action = data["action"]
                subscription_id = decrypt(subscriber.subscription_id)
                if action == None:
                    details = show_sub_details(subscription_id)
                    paypal_status = details["status"]
                    response = {
                        "method": method,
                        "subscribed": subscriber.is_subscribed,
                        "status": paypal_status,
                    }
                    return JSONResponse(content=response, status_code=status.HTTP_200_OK)
                elif action == "Stop":
                    suspend_sub(subscription_id)
                    details = show_sub_details(subscription_id)
                    paypal_status = details["status"]
                    response = {
                        "method": method,
                        "subscribed": subscriber.is_subscribed,
                        "status": paypal_status,
                    }
                    return JSONResponse(content=response, status_code=status.HTTP_200_OK)
                elif action == "Re-start":
                    activate_sub(subscription_id)
                    details = show_sub_details(subscription_id)
                    paypal_status = details["status"]
                    response = {
                        "method": method,
                        "subscribed": subscriber.is_subscribed,
                        "status": paypal_status,
                    }
                    return JSONResponse(content=response, status_code=status.HTTP_200_OK)
