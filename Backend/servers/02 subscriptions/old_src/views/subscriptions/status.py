from fastapi import Request, status
from fastapi.responses import JSONResponse
from dotenv import load_dotenv
import stripe
import os
from src.datatypes.urls import StripeUrls
from src.queries.subscriptions.get import get_subscriber
from src.models.subscriber import Subscriber
from src.services.stripe_service import StripeService

load_dotenv()

async def retrieve_status(request: Request, subscription: StripeUrls | None):
    user_uuid = request.state.user_uuid

    subscriber: Subscriber | None = get_subscriber(user_uuid)
    if subscriber == None:
        response = {"error", "Invalid token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    if subscriber.is_subscribed == False:
        stripe_service = StripeService()
        customer = stripe.Customer.create()

        if subscription == None:
            response = {"error", "No urls"}
            return JSONResponse(
                content=response, status_code=status.HTTP_400_BAD_REQUEST
            )
        success_url = subscription.success_url
        cancel_url = subscription.cancel_url
        stripe_checkout = stripe_service.build_checkout_session(
            subscriber, customer, success_url, cancel_url
        )
        stripe_url = stripe_checkout.url

        response = {
            "subscribed": subscriber.is_subscribed,
            "trial": subscriber.has_trial,
            "stripe_customer_id": customer.id,
            "stripe_url": stripe_url,
        }
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)
    else:
        response = {
            "subscribed": subscriber.is_subscribed,
            "trial": subscriber.has_trial,
        }
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)
