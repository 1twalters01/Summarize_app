from enum import Enum
from datetime import datetime

class Error():
    reason: str

class PaymentMethodEnum(Enum):
    Stripe = 1
    Paypal = 2
    Crypto = 3

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

class Subscriber():
    id: int
    user_id: int
    customer_id: str|None
    payment_method: PaymentMethodEnum

async def create_subscription_urls_view(request: Request):
    user_uuid = request.state.user_uuid

    subscription_metadata: SubscriptionMetadata|Error = get_or_create_subscription_metadata_for_user_uuid(user_uuid)
    if type(subscription_metadata) == Error:
        raise HTTPException(
            status_code=400,
            detail=Error
        )
    is_subscribed = check_and_update_subscription_status(subscription_metadata)
    if is_subscribed == true:
        return JSONResponse(content=response, status_code=status.HTTP_409_CONFLICT)

    # Create stripe url
    stripe_subscriber: Subscriber|None|Error = get_subscriber_for_user_uuid(user_uuid, PaymentMethodEnum.Stripe)
    if type(stripe_subscriber) == Error:
        raise HTTPException(
            status_code=400,
            detail=Error
        )
    if not stripe_subscriber:
        stripe_subscriber = create_subscriber_for_user_uuid(user_uuid, PaymentMethodEnum.Stripe)

    if stripe_subscriber.customer_id:
        if validate_stripe_customer_id(stripe_subscriber.customer_id) == False:
            response = {"error": "Invalid stripe customer ID"}
            return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)
        else:
            stripe_customer_id = generate_stripe_customer_id
            set_stripe_customer_id(stripe_subscriber, stripe_customer_id) # Check error
            stripe_subscriber.customer_id = stripe_customer_id
        
    stripe_url = create_stripe_subscription_checkout_session(stripe_subscriber).url


    # Create paypal url
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

    # Create crypto url
    
    # Send response
    response = {
        "stripe_url": stripe_url,
        "paypal_url": paypal_url,
    }
    return response