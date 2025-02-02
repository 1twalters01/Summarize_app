from fastapi import APIRouter
from src.views.webhooks.stripe import stripe_webhook as stripe_webhook_view
from src.views.webhooks.paypal import paypal_webhook as paypal_webhook_view

router = APIRouter()

router.add_api_route("/webhooks/stripe", stripe_webhook_view, methods=["POST"])
router.add_api_route("/webhooks/paypal", paypal_webhook_view, methods=["POST"])
