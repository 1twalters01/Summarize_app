from fastapi import Request, status
from src.services import subscription_service, stripe_service, paypal_service
from src.datatypes.subscription_method import SubscriptionMethodEnum

async def get_invoices_view(request: Request):
    user_uuid = request.state.user_uuid
    subscribers = subscription_service.subscribers(user_uuid)

    paypal_encrypted_customer_ids = [
        subscriber.encrypted_id
        for subscriber in subscribers
        if subscriber.subscription_method_enum == SubscriptionMethodEnum.Paypal
    ]
    stripe_encrypted_customer_ids = [
        subscriber.encrypted_id
        for subscriber in subscribers
        if subscriber.subscription_method_enum == SubscriptionMethodEnum.Stripe
    ]

    paypal_invoice_data = []
    for encrypted_customer_id in paypal_encrypted_customer_ids:
        paypal_invoice_data += paypal_service.get_paypal_invoices_data(encrypted_customer_id)
    stripe_invoice_data = []
    for encrypted_customer_id in stripe_encrypted_customer_ids:
        stripe_invoice_data += stripe_service.get_stripe_invoices_data(encrypted_customer_id)

    invoice_data = sorted(
        stripe_invoice_data + paypal_invoice_data,
        key=lambda x: x[1],
    )

    if not invoices:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="No invoices found",
        )

    return {"invoices": invoice_data}
