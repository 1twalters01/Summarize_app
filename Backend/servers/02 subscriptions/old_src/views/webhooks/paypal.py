async def paypal_webhook(event: PaypalEvent):
    if event.event_type == "BILLING.SUBSCRIPTION.ACTIVATED":
        pass
    if event.event_type == "PAYMENT.SALE.COMPLETED":
        pass
    elif event.event_type == "Billing.subscription.expired":
        pass
    else:
        pass
