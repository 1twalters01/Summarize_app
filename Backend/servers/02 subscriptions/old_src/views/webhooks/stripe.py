async def stripe_webhook(event: StripeEvent):
    if event.type == "customer.deleted":
        pass
    elif event.type == "customer.subscription.trial_will_end":
        pass
    elif event.type == "invoice.payment_succeeded":
        pass
    else:
      pass
