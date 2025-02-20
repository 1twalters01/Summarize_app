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

async def resume_subscription_view(request: Request, reason: str = None):
    user_uuid = request.state.user_uuid

    last_subscription: Subscription|None = get_last_subscription_from_subscription_history_for_user_uuid(user_uuid)
    if not last_subscription:
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)
    
    resume_subscription(last_subscription)

    payment_method: PaymentMethodEnum = get_payment_method_from_subscription(last_subscription)
    if payment_method == PaymentMethodEnum.Stripe:
        stripe_service.resume_subscription(last_subscription.subscriber_id, reason)
    if payment_method == PaymentMethodEnum.Paypal:
        paypal_service.activate_sub(last_subscription.subscriber_id, reason)
    if payment_method == PaymentMethodEnum.Crypto:
        resume_subscription(last_subscription)
        pass
    
    response = {
        "success": True
    }
    return response