from datetime import datetime
from src.datatypes.payment_type import PaymentTypeEnum
from src.datatypes.payment_method import PaymentMethodEnum
from src.datatypes.subscription import Subscription

async def resume_subscription_view(request: Request, reason: str = None):
    user_uuid = request.state.user_uuid

    last_subscription: Subscription|None = get_last_subscription_from_subscription_history_for_user_uuid(user_uuid)
    if not last_subscription:
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    payment_method: PaymentMethodEnum = get_payment_method_from_subscription(last_subscription)
    if payment_method == PaymentMethodEnum.Stripe:
        stripe_service.resume_subscription(last_subscription.subscriber_id, reason)
    if payment_method == PaymentMethodEnum.Paypal:
        paypal_service.activate_sub(last_subscription.subscriber_id, reason)
    if payment_method == PaymentMethodEnum.Crypto:
        crypto_service.resume_subscription(last_subscription)
        pass
    
    resume_subscription(last_subscription)
    
    response = {
        "success": True
    }
    return response