from fastapi import APIRouter
from src.views.webhooks.stripe import stripe_webhook_handler
from src.views.webhooks.paypal import paypal_webhook_handler
from src.views.webhooks.crypto import crypto_webhook_handler

router = APIRouter()

# Webhooks for Stripe
router.add_api_route(
    "/webhooks/stripe",
    stripe_webhook_handler,
    methods=["POST"],
)

# Webhooks for PayPal
router.add_api_route(
    "/webhooks/paypal",
    paypal_webhook_handler,
    methods=["POST"],
)

# Webhooks for Cryptocurrency payments
router.add_api_route(
    "/webhooks/crypto",
    crypto_webhook_handler,
    methods=["POST"],
)
