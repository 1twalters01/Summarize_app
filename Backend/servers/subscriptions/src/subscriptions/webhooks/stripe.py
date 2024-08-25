from datetime import datetime
from dateutil.relativedelta import relativedelta
from fastapi import Request, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel
from ...utils.encryption import decrypt, encrypt

class StripeObject(BaseModel):
    id: str
    customer: str

class StripeData(BaseModel):
    object: StripeObject

class StripeEvent(BaseModel):
    data: StripeData
    type: str


def get_uuid_from_customer_id(customer_id: str):
    # user_id = "Select uuid from subscribers where customer_id=encrypt(customer_id) and methods.payment_method=stripe" (sql join)
    user_uuid: str|None = customer_id
    return user_uuid

async def stripe_webhook(event: StripeEvent):
    if event.type == "customer.deleted":
        customer_id = event.data.object.id
        user_uuid = get_uuid_from_customer_id(customer_id)
        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR)
        # set subscribed=False where user_uuid=user_uuid
        response = {"success": True}
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)

    elif event.type == "customer.subscription.trial_will_end":
        customer_id = event.data.object.customer
        user_uuid = get_uuid_from_customer_id(customer_id)
        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR)  
        # SELECT email from users where uuid=user_uuid
        # Email user with email
        response = {"success": True}
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)

    elif event.type == "invoice.payment_succeeded":
        customer_id = event.data.object.customer
        user_uuid = get_uuid_from_customer_id(customer_id)
        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR)
        subscription_id = event.data.object.id
        start_date = datetime.now()
        # SELECT trial_status from users where uuid=user_uuid
        trial_status = False 

        if trial_status == False:
            end_date = datetime.now() + relativedelta(months=1, days=2)
        else:
            end_date = datetime.now() + relativedelta(months=1, weeks=2, days=2)

        # set customer_id=customer_id, subscription_id=subscription_id, start_date=start_date, trial_status=False, end_date=end_date, subscribed=true WHERE uuid=user_uuid
        response = {"success": True}
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)

    else:
        response = {"success": False}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

