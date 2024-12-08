from fastapi import Request, status
from fastapi.responses import JSONResponse
from dotenv import load_dotenv
from src.models.subscriber import Subscriber
from src.queries.subscriptions.get import get_or_create_subscriber
from src.queries.subscriptions.save import save_subscriber
from src.services.stripe_service import StripeService
from src.services.encryption_service import EncryptionService
from src.services.paypal_service import PayPalSubscriptionService
import stripe
import os

load_dotenv()


def obtain_method(subscriber):
    if subscriber:
        method = str(subscriber.method)
    else:
        method = None
    return method


async def get_premium_status(request: Request, data):
    user_uuid = request.state.user_uuid

    subscriber: Subscriber | None = get_or_create_subscriber(user_uuid)
    if subscriber == None:
        response = {"error", "Invalid token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)
    method = obtain_method(subscriber)
    stripe_service = StripeService()
    if subscriber.is_subscribed == False:
        if data["method"] == None:
            success_url = data["success_url"]
            cancel_url = data["cancel_url"]
            customer = stripe.Customer.create()
            stripe_url = stripe_service.build_checkout_session(
                subscriber, customer, success_url, cancel_url
            ).url

            response = {
                "subscribed": subscriber.is_subscribed,
                "trial": subscriber.has_trial,
                "stripe_customer_id": customer.id,
                "stripe_url": stripe_url,
            }
            return JSONResponse(content=response, status_code=status.HTTP_200_OK)

        if data["method"] == "Stripe":
            if subscriber.is_subscribed == False:
                customer_id = data["customer_id"]
                try:
                    save_subscriber(
                        user_uuid,
                        subscriber,
                        method=data["method"],
                        customer_id=customer_id,
                    )
                except:
                    error = {"error": "Stripe customer id was not found"}
                    return JSONResponse(
                        content=error, status_code=status.HTTP_404_NOT_FOUND
                    )
                response = {"success": "User created successfully"}
                return JSONResponse(content=response, status_code=status.HTTP_200_OK)

        if data["method"] == "Paypal":
            if subscriber.is_subscribed == False:
                subscriber_id = data.get("subscriber_id")
                try:
                    save_subscriber(
                        user_uuid,
                        subscriber,
                        method=data["method"],
                        subscriber_id=subscriber_id,
                    )
                except:
                    error = "Paypal customer id was not found"
                    return JSONResponse(
                        content=error, status_code=status.HTTP_404_NOT_FOUND
                    )
                response = "User created successfully"
                return JSONResponse(content=response, status_code=status.HTTP_200_OK)

    else:
        method = subscriber.payment_method
        if subscriber.is_subscribed == True:
            if method == "Stripe":
                stripe.api_key = os.getenv("STRIPE_SECRET_KEY")
                return_url = data["return_url"]
                customer_id = subscriber.customer_id
                if not customer_id:
                    error = "Id error"
                    return JSONResponse(
                        content=error, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
                    )
                stripe_portal = stripe_service.build_portal_session(
                    customer_id, return_url
                )
                response = {
                    "method": method,
                    "subscribed": subscriber.is_subscribed,
                    "url": stripe_portal.url,
                }
                return JSONResponse(content=response, status_code=status.HTTP_200_OK)

            if method == "Paypal":
                paypal = PayPalSubscriptionService()
                action = data["action"]
                encryption_service = EncryptionService()
                subscription_id = encryption_service.decrypt(subscriber.subscription_id)
                if action == None:
                    details = paypal.show_sub_details(subscription_id)
                    paypal_status = details["status"]
                    response = {
                        "method": method,
                        "subscribed": subscriber.is_subscribed,
                        "status": paypal_status,
                    }
                    return JSONResponse(
                        content=response, status_code=status.HTTP_200_OK
                    )
                elif action == "Stop":
                    paypal.suspend_sub(subscription_id)
                    details = paypal.show_sub_details(subscription_id)
                    paypal_status = details["status"]
                    response = {
                        "method": method,
                        "subscribed": subscriber.is_subscribed,
                        "status": paypal_status,
                    }
                    return JSONResponse(
                        content=response, status_code=status.HTTP_200_OK
                    )
                elif action == "Re-start":
                    paypal.activate_sub(subscription_id)
                    details = paypal.show_sub_details(subscription_id)
                    paypal_status = details["status"]
                    response = {
                        "method": method,
                        "subscribed": subscriber.is_subscribed,
                        "status": paypal_status,
                    }
                    return JSONResponse(
                        content=response, status_code=status.HTTP_200_OK
                    )
