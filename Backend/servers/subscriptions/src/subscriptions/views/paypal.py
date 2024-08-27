from fastapi import Request, status
from pydantic import BaseModel

async def is_user_subscribed(user_uuid):
    # get is_subscribed in users from user_uuid
    # return is_subscribed

async def create_stripe_customer(request: Request, stripe_customer_id: str):
    headers = request.headers
    bearer = headers.get("bearer_token")
    # get claims from token
    # get user_uuid
    is_subscribed: bool = is_user_subscribed(user_uuid)

    if is_subscribed == True:
        response = {"error": "Customer already exists"}
        return JSONResponse(content=response, status_code=status.HTTP_409_CONFLICT)

    # Validate paypal_customer_ID
    try:
        # SET subscription_id=encrypt(subscriber_id), customer_id=NULL), m.method="Paypal" INTO subscriptions where user_uuid=user_uuid (joins, fix this)
        response = {"success": true}
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)
    else:
        return JSONResponse(content=response, status_code=status.HTTP_400_SERVER_ERROR)
      
        subscriber.method = self.payment_method(method)
        subscriber.customer_id = None
        subscriber.subscription_id = encrypt(subscriber_id)
