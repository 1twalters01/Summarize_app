from fastapi import Request, status
from pydantic import BaseModel
import stripe

class Subscription(BaseModel):
    success_url: str
    cancel_url: str

async def is_user_subscribed(user_uuid):
    # get is_subscribed in users from user_uuid
    # return is_subscribed
  
async def RetrieveStatus(subscription: Subscription):
    headers = request.headers
    bearer = headers.get("bearer_token")
    # get claims from token
    # get user_uuid
  
    is_subscribed: bool = is_user_subscribed(user_uuid)

    success_url = subscription.success_url
    cancel_url = subscription.cancel_url

    if is_subscribed == False:
        stripe.api_key = STRIPE_SECRET_KEY # Get from env
        customer = stripe.Customer.create()
        stripe_checkout = self.build_stripe_checkout(subscriber, customer, success_url, cancel_url)
        stripe_url = stripe_checkout.url

        charge = self.build_coinbase_checkout(subscriber, success_url, cancel_url)
        coinbase_url = charge.hosted_url

        response = {
            'subscribed':subscribed,
            'trial':subscriber.trial,
            'stripe_customer_id':customer.id,
            'stripe_url':stripe_url,
            'coinbase_url':coinbase_url
        }
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)
    else:
        response = {
            'subscribed':subscribed,
            'trial':subscriber.trial,
        }
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)
