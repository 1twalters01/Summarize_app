from datetime import datetime
from dateutil.relativedelta import relativedelta
from fastapi import Request, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel
from starlette.status import HTTP_200_OK
from ...utils.encryption import decrypt, encrypt


class StripeObject(BaseModel):
    id: str
    customer: str


class PaypalResource(BaseModel):
    id: str


class PaypalEvent(BaseModel):
    event_type: str
    resource: PaypalResource


async def paypal_webhook(event: PaypalEvent):
    if event.event_type == "BILLING.SUBSCRIPTION.ACTIVATED":
        print("Billing.subscription.activated")  # log event
        subscription_id = event.resource.id  # Encrypt from here?
        user_uuid = get_subscriber_or_none(subscription_id, "Paypal")
        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(
                content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
            )

        # get current end date
        end_date = get_end_date(user_uuid)
        if subscriber.trial == False:
            end_date = max(end_date, datetime.now() + relativedelta(months=1, days=2))
        else:
            end_date = max(
                end_date, datetime.now() + relativedelta(months=1, weeks=2, days=2)
            )

        subscription_id = encrypt(subscription_id)  # Move up?
        start_date = datetime.now()
        # set subscription_id=subscription_id, start_date=start_date, trial_status=False, end_date=end_date, subscribed=true, methods.payment_method=paypal WHERE uuid=user_uuid
        response = {"success": True}
        return JSONResponse(response, status_code=status.HTTP_200_OK)

    # Whenever I get paid
    elif event.event_type == "PAYMENT.SALE.COMPLETED":
        print("PAYMENT.SALE.COMPLETED")  # log event
        subscription_id = event.resource.id  # Encrypt here?
        user_uuid = get_subscriber_or_none(subscription_id, "Paypal")
        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(
                content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
            )

        # get current end date
        end_date = get_end_date(user_uuid)
        if subscriber.trial == False:
            end_date = max(end_date, datetime.now() + relativedelta(months=1, days=2))
        else:
            end_date = max(
                end_date, datetime.now() + relativedelta(months=1, weeks=2, days=2)
            )

        subscriber.subscription_id = encrypt(subscription_id)  # Move encryption up?
        subscriber.start_date = date.today()
        # set subscription_id=subscription_id, start_date=start_date, trial_status=False, end_date=end_date, subscribed=true, methods.payment_method=paypal WHERE uuid=user_uuid
        response = {"success": True}
        return JsonResponse(response, status=200)

    # If it becomes expired
    elif event.event_type == "Billing.subscription.expired":
        print("Billing.subscription.expired")  # log event
        subscription_id = event.resource.id
        user_uuid = get_subscriber_or_none(subscription_id, "Paypal")
        if user_uuid == None:
            response = {"success": False}
            return JSONResponse(
                content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
            )

        # set subscription_id=NULL, start_date=NULL, trial_status=False, end_date=NULL, subscribed=false, methods.payment_method=NULL WHERE uuid=user_uuid
        response = {"success": True}
        return JSONResponse(response, status_code=HTTP_200_OK)

    else:
        response = {"success": False}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)
