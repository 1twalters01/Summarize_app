from datetime import datetime

class PaymentTierEnum():
    Subscription_Monthly = 1
    Subscription_Yearly = 2
    Payment_1_Month = 3
    Payment_3_Months = 4
    Payment_1_Yea = 5

class SubscriptionMetadata():
    user_id: int
    payment_tier_enum: PaymentTierEnum|None
    has_trial: bool
    trial_start_date: datetime|None
    trial_end_date: datetime|None

class Subscription():
    id: int
    subscriber_id: int
    subscription_id: str
    payment_tier_enum: PaymentTierEnum
    subscription_start_date: datetime
    subscription_end_date: datetime|None
    cancellation_date: datetime|None

class PaymentMethodEnum(Enum):
    Stripe = 1
    Paypal = 2
    Crypto = 3

async def change_subscription_view(request: Request, reason: str = None):
    user_uuid = request.state.user_uuid

    last_subscription: Subscription|None = get_last_subscription_from_subscription_history_for_user_uuid(user_uuid)
    if not last_subscription:
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    payment_method: PaymentMethodEnum = get_payment_method_from_subscription(last_subscription)
    if payment_method == PaymentMethodEnum.Stripe:
        stripe_service.update_subscription(last_subscription.subscriber_id, reason)
        response = {
            "success": True
        }
        return response

    if payment_method == PaymentMethodEnum.Paypal:
        # Cancel current plan
        cancel_subscription(last_subscription)
        paypal_service.cancel_sub(last_subscription.subscriber_id, reason)

        # Create new plan
        paypal_url = None
        paypal_subscriber: Subscriber|None|Error = get_subscriber_for_user_uuid(user_uuid, PaymentMethodEnum.Paypal)
        if type(stripe_subscriber) == Error:
            raise HTTPException(
                status_code=400,
                detail=Error
            )
        if not stripe_subscriber:
            paypal_subscriber = create_subscriber_for_user_uuid(user_uuid, PaymentMethodEnum.Paypal)

        if paypal_subscriber.customer_id:
            if validate_paypal_customer_id(paypal_subscriber.customer_id) == False:
                response = {"error": "Invalid paypal customer ID"}
                return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

        paypal_subscription = create_paypal_subscription
        set_paypal_customer_id(paypal_subscriber, paypal_subscription["subscriber"].payer_id)
        for link in paypal_subscription["links"]:
            if link["rel"] == "approve":
                paypal_url = link["rel"]
        
        if not paypal_url:
            return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)
        
        response = {
            "success": true,
            "link": paypal_url,
        }
    if payment_method == PaymentMethodEnum.Crypto:
        pass
    
    response = {
        "success": True
    }
    return response