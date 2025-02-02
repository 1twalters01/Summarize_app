import stripe
from src.models.subscriber import Subscriber
from src.services.encryption_service import EncryptionService
import os
from dotenv import load_dotenv

load_dotenv()


class StripeService:
    def __init__(self):
        stripe.api_key = os.getenv("STRIPE_SECRET_KEY")

    def build_portal_session(self, encrypted_customer_id: str, return_url: str):
        """Create a Stripe billing portal session."""
        encryption_service = EncryptionService()
        customer_id = encryption_service.decrypt(encrypted_customer_id)
        portalSession = stripe.billing_portal.Session.create(
            customer=customer_id,
            return_url=return_url,
        )
        return portalSession

    def build_checkout_session(
        self,
        subscriber: Subscriber,
        customer: stripe.Customer,
        success_url: str,
        cancel_url: str,
    ):
        """Create a Stripe checkout session."""
        prices = stripe.Price.list(
            lookup_keys=["Summarize Premium"], expand=["data.product"]
        )
        if not prices.data:
            raise Exception("No price data found for lookup key")

        line_items = [
            {
                "price": prices.data[0].id,
                "quantity": 1,
            },
        ]

        checkout_kwargs = {
            "line_items": line_items,
            "customer": customer,
            "mode": "subscription",
            "success_url": success_url,
            "cancel_url": cancel_url,
        }

        if subscriber.has_trial == True:
            checkout_kwargs["subscription_data"] = {"trial_period_days": 7}

        checkout_session = stripe.checkout.Session.create(**checkout_kwargs)
        return checkout_session
