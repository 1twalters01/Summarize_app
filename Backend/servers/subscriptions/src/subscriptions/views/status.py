from fastapi import Request, status
from fastapi.responses import JSONResponse
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
        stripe_checkout = build_stripe_checkout(subscriber, customer, success_url, cancel_url)
        stripe_url = stripe_checkout.url

        charge = build_coinbase_checkout(subscriber, success_url, cancel_url)
        coinbase_url = charge.hosted_url

        response = {
            'subscribed':is_subscribed,
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

def build_stripe_checkout(self, subscriber, customer, success_url, cancel_url):
    if not subscriber:
        error = 'Subscriber does not exist)'
        raise Exception(error)
            

    prices = stripe.Price.list(
        lookup_keys=['Conjugat Premium'],
        expand=['data.product']
    )

    line_items=[
        {
            'price': prices.data[0].id,
            'quantity': 1,
        },
    ]

    checkout_kwargs = {
        'line_items' : line_items,
        'customer':customer,
        'mode':'subscription',
        'success_url':success_url,
        'cancel_url':cancel_url,
    }

    if subscriber.trial == True:
        checkout_kwargs['subscription_data'] = {'trial_period_days':7}

    checkout_session = stripe.checkout.Session.create(**checkout_kwargs)
    return checkout_session

def build_coinbase_checkout(self, subscriber, success_url, cancel_url):
    if not subscriber:
        error = 'Subscriber does not exist)'
        raise Exception(error)

    client = Client(api_key=settings.COINBASE_COMMERCE_API_KEY)

    checkout_kwargs = {
        'name':'Conjugat Premium',
        'local_price': {
            'currency':'GBP'
        },
        'pricing_type':'fixed_price',
        'rediret_url':success_url,
        'cancel_url':cancel_url,
    }

    if subscriber.trial == True:
        checkout_kwargs['description'] = '1 Week of conjugat Premium'
        checkout_kwargs['local_price']['amount'] = '0.01'

    else:
        checkout_kwargs['description'] = '1 Month of conjugat Premium'
        checkout_kwargs['local_price']['amount'] = '3.00'

    charge = client.charge.create(**checkout_kwargs)
    return charge
