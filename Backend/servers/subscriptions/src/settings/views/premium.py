from fastapi import Depends, Request, status
from fastapi.responses import JSONResponse
from ...utils.encryption import decrypt, encrypt
from ...utils.encryption import decrypt
from ...utils.paypal import show_sub_details, suspend_sub, activate_sub, cancel_sub
import stripe
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


def does_subscriber_exist(self, user):
    try:
        subscriber = UserProfile.objects.get(user=user)
    except:
        subscriber = UserProfile.objects.create(
            user=user, method_id=self.payment_method("None")
        )
        subscriber.save()
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

def payment_method(method):
    if method == "Stripe":
        return 1
    elif method == "Paypal":
        return 2
    elif method == "Coinbase":
        return 3
    elif method == "None":
        return 4

def save_subscriber(self, user, subscriber, customer_id):
    if not subscriber:
        subscriber = UserProfile.objects.create(
                user=user, method_id=self.payment_method("Stripe")
                )
    subscriber.method_id = self.payment_method("Stripe")
    # Reset the subscription and customer ids
    subscriber.subscription_id = None
    subscriber.customer_id = encrypt(customer_id)
    subscriber.save()

def build_stripe_portal(stripe, subscriber, return_url):
    customer = decrypt(subscriber.customer_id)
    portalSession = stripe.billing_portal.Session.create(
            customer=customer,
            return_url=return_url,
            )
    return portalSession

def get_premium_status(self, data):
    user = self.context["user"]
    stripe.api_key = os.getenv("STRIPE_SECRET_KEY")
    subscriber = self.does_subscriber_exist(user)
    method = self.obtain_method(subscriber)
    subscribed = self.is_user_subscribed(user, subscriber)
    if subscribed == False:
        if data["method"] == None:
            success_url = data["success_url"]
            cancel_url = data["cancel_url"]
            customer = stripe.Customer.create()
            stripe_url = self.build_stripe_checkout(
                    subscriber, customer, success_url, cancel_url
                    ).url

            charge = self.build_coinbase_checkout(
                    subscriber, success_url, cancel_url
                    )
            coinbase_url = charge.hosted_url

            response = {
                    "subscribed": subscribed,
                    "trial": subscriber.trial,
                    "stripe_customer_id": customer.id,
                    "stripe_url": stripe_url,
                    "coinbase_url": coinbase_url,
                    }
            return JSONResponse(content=response, status_code=status.HTTP_200_OK)

        if data["method"] == "Stripe":
            user = self.context["user"]
            subscriber = self.does_subscriber_exist(user)
            subscribed = self.is_user_subscribed(user, subscriber)
            if subscribed == False:
                customer_id = data["customer_id"]
                try:
                    self.save_subscriber(user, subscriber, customer_id)
                except:
                    error = {"error": "Stripe customer id was not found"}
                    return JSONResponse(content=error, status_code=status.HTTP_404_NOT_FOUND)
                response = {"success": "User created successfully"}
                return JSONResponse(content=response, status_code=status.HTTP_200_OK)

        if data["method"] == "Paypal":
            user = self.context["user"]
            subscriber = self.does_subscriber_exist(user)
            subscribed = self.is_user_subscribed(user, subscriber)
            if subscribed == False:
                subscriber_id = data.get("subscriber_id")
                try:
                    self.save_subscriber("Paypal", user, subscriber, subscriber_id)
                except:
                    error = "Paypal customer id was not found"
                    return JSONResponse(content=error, status_code=status.HTTP_404_NOT_FOUND)
                response = "User created successfully"
                return JSONResponse(content=response, status_code=status.HTTP_200_OK)

    else:
        user = self.context["user"]
        stripe.api_key = os.getenv("STRIPE_SECRET_KEY")
        subscriber = self.does_subscriber_exist(user)
        method = self.obtain_method(subscriber)
        subscribed = self.is_user_subscribed(user, subscriber)
        if subscribed == True:
            subscriber.url = None
            subscriber.status = None

            if method == "Stripe":
                return_url = data["return_url"]
                stripe_portal = self.build_stripe_portal(
                        stripe, subscriber, return_url
                        )
                response = {
                        "method": method,
                        "subscribed": subscribed,
                        "url": stripe_portal.url,
                        }
                return JSONResponse(content=response, status_code=status.HTTP_200_OK)

            if method == "Paypal":
                action = data["action"]
                subscription_id = decrypt(subscriber.subscription_id)
                if action == None:
                    details = show_sub_details(subscription_id)
                    subscriber.status = details["status"]
                    response = {
                            "method": method,
                            "subscribed": subscribed,
                            "status": subscriber.status,
                            }
                    return JSONResponse(content=response, status_code=status.HTTP_200_OK)
                elif action == "Stop":
                    suspend_sub(subscription_id)
                    details = show_sub_details(subscription_id)
                    subscriber.status = details["status"]
                    response = {
                            "method": method,
                            "subscribed": subscribed,
                            "status": subscriber.status,
                            }
                    return JSONResponse(content=response, status_code=status.HTTP_200_OK)
                elif action == "Re-start":
                    activate_sub(subscription_id)
                    details = show_sub_details(subscription_id)
                    subscriber.status = details["status"]
                    response = {
                            "method": method,
                            "subscribed": subscribed,
                            "status": subscriber.status,
                            }
                    return JSONResponse(content=response, status_code=status.HTTP_200_OK)
