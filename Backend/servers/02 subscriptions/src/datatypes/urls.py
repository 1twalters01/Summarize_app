from pydantic import BaseModel


class StripeUrls(BaseModel):
    success_url: str
    cancel_url: str
