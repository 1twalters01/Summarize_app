from fastapi import APIRouter
from src.views.webhooks.stripe import stripe_webhook
from src.views.webhooks.paypal import paypal_webhook
# from .webhooks.coinbase import coinbase_webhook

router = APIRouter()

router.add_api_route("/webhooks/stripe", stripe_webhook, methods=["POST"])
router.add_api_route("/webhooks/paypal", paypal_webhook, methods=["POST"])



