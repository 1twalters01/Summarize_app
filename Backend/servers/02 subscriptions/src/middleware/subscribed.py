load_dotenv()

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

def is_authenticated_middleware(request: Request):
    user_uuid = request.state.user_uuid

    subscription_metadata: SubscriptionMetadata|Error = get_or_create_subscription_metadata_for_user_uuid(user_uuid)
    if type(subscription_metadata) == Error | subscription_metadata.payment_tier_enum == None:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Authorization token must be prefixed with 'Bearer '",
        )

    is_subscribed = check_and_update_subscription_status(subscription_metadata)
    if is_subscribed == false:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Authorization token must be prefixed with 'Bearer '",
        )