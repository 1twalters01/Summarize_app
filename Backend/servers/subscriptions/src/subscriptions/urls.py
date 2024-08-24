from fastapi import APIRouter
from .webhooks.stripe import stripe_webhook

router = APIRouter()

router.add_api_route("/webhooks/stripe", stripe_webhook, methods=["POST"])
