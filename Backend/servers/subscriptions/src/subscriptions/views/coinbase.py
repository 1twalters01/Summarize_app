from fastapi import Request, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel

async def is_user_subscribed(user_uuid):
    # get is_subscribed in users from user_uuid
    is_subscribed = True
    return is_subscribed

async def create_coinbase_customer(request: Request, stripe_customer_id: str):
    headers = request.headers
    bearer = headers.get("bearer_token")
    # get claims from token
    # get user_uuid
    is_subscribed: bool = is_user_subscribed(user_uuid)

    if is_subscribed == True:
        response = {"error": "Customer already exists"}
        return JSONResponse(content=response, status_code=status.HTTP_409_CONFLICT)

    # Validate Coinbase_Subscription_ID
    try:
        # SET subscription_id=encrypt(subscription_id, customer_id=NULL, m.method="Coinbase" INTO subscriptions where user_uuid=user_uuid (joins, fix this)
        response = {"success": True}
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)
    except:
        response = {"success": False}
        return JSONResponse(content=response, status_code=status.HTTP_400_SERVER_ERROR)
