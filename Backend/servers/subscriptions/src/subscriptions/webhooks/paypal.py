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
        req_userprofile = get_subscriber_or_none(subscription_id, 'Paypal')
        if req_userprofile:
            # Add the subscription id to the database
            user = req_userprofile.user
            subscriber = UserProfile.objects.get(user=user)
            subscriber.subscription_id = encrypt(subscription_id)
            subscriber.subscribed = True
            subscriber.start_date = date.today()
            if subscriber.trial == False:
                subscriber.end_date = date.today() + relativedelta(months=1) + relativedelta(days=2)
            else:
                subscriber.end_date = date.today() + relativedelta(days = 7)
            subscriber.trial = False
            subscriber.save()
            return JsonResponse({"success": True}, status=200)
        return JsonResponse({"success": False}, status=500)
    

    # Whenever I get paid
    elif event.event_type == "PAYMENT.SALE.COMPLETED":
        print("PAYMENT.SALE.COMPLETED") # log event
        subscription_id = event.resource.id
        req_userprofile = get_subscriber_or_none(subscription_id, 'Paypal')
        if req_userprofile:
            user = req_userprofile.user
            subscriber = UserProfile.objects.get(user=user)
            subscriber.subscription_id = encrypt(subscription_id)
            subscriber.subscribed = True
            subscriber.start_date = date.today()
            if subscriber.trial == False:
                subscriber.end_date = date.today() + relativedelta(months=1) + relativedelta(days=2)
            else:
                subscriber.end_date = date.today() + relativedelta(days = 7)
            subscriber.trial = False
            subscriber.save()
            return JsonResponse({"success": True}, status=200)
        return JsonResponse({"success": False}, status=500)


    # If it becomes expired
    elif event.event_type == "Billing.subscription.expired":
        print("Billing.subscription.expired") # log event
        subscription_id = event.resource.id
        req_userprofile = get_subscriber_or_none(subscription_id, 'Paypal')
        if req_userprofile:
            user = req_userprofile.user
            subscriber = UserProfile.objects.get(user=user)
            subscriber.subscribed = False
            subscriber.trial = False
            subscriber.save()
            return JsonResponse({"success": True}, status=200)
        return JsonResponse({"success": False}, status=500)
