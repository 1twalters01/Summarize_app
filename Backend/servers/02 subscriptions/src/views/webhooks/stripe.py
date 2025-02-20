from pydantic import BaseModel

class StripeObject(BaseModel):
    id: str
    customer: str


class StripeData(BaseModel):
    object: StripeObject


class StripeEvent(BaseModel):
    data: StripeData
    type: str
    
async def stripe_webhook(event: StripeEvent):
    if event.type == "customer.deleted":
        pass
    elif event.type == "customer.subscription.trial_will_end":
        pass
    elif event.type == "invoice.payment_succeeded":
        pass
    else:
      pass
