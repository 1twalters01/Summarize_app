from datetime import datetime
from dateutil.relativedelta import relativedelta
from fastapi import Request, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel
from ...utils.encryption import decrypt, encrypt

class StripeObject(BaseModel):
    id: str
    customer: str

class PaypalResource(BaseModel):
    id: str

class PaypalEvent(BaseModel):
    event_type: str
    resource: PaypalResource

async def paypal_webhooks(event: PaypalEvent):
    if event.event_type == "BILLING.SUBSCRIPTION.ACTIVATED":
        print("Billing.subscription.activated") # log event
        subscription_id = event.resource.id
        user_uuid = get_subscriber_or_none(subscription_id, 'Paypal')
        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR)
    
        # Add the subscription id to the database
        # get current end date
        end_date = get_end_date(user_uuid)
        if subscriber.trial == False:
            end_date = max(end_date, datetime.now() + relativedelta(months=1, days=2))
        else:
            end_date = max(end_date, datetime.now() + relativedelta(months=1, weeks=2, days=2))
    
        subscription_id = encrypt(subscription_id)
        start_date = datetime.now()
        # set subscription_id=subscription_id, start_date=start_date, trial_status=False, end_date=end_date, subscribed=true WHERE uuid=user_uuid
        response = {"success": True}
        return JsonResponse(response, status=200)
    
    # Whenever I get paid
    elif event.event_type == "PAYMENT.SALE.COMPLETED":
        print("PAYMENT.SALE.COMPLETED") # log event
        subscription_id = event.resource.id
        user_uuid = get_subscriber_or_none(subscription_id, 'Paypal')
    
        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR)
    
        user = req_userprofile.user
        subscriber = UserProfile.objects.get(user=user)
        subscriber.subscription_id = encrypt(subscription_id)
        subscriber.subscribed = True
        subscriber.start_date = date.today()
        if subscriber.trial == False:
            subscriber.end_date = date.today() + relativedelta(months=1, days=2)
        else:
            subscriber.end_date = date.today() + relativedelta(months=1, weeks=2, days=2)
        subscriber.trial = False
        subscriber.save()
        response = {"success": True}
        return JsonResponse(response, status=200)


    # If it becomes expired
    elif event.event_type == "Billing.subscription.expired":
        print("Billing.subscription.expired") # log event
        subscription_id = event.resource.id
        req_userprofile = get_subscriber_or_none(subscription_id, 'Paypal')

        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR)
    
        user = req_userprofile.user
        subscriber = UserProfile.objects.get(user=user)
        subscriber.subscribed = False
        subscriber.trial = False
        subscriber.save()
        response = {"success": True}
        return JsonResponse(response, status=200)
        
    else:
        response = {"success": False}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)
