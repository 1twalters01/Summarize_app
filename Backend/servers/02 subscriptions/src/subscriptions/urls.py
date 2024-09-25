from fastapi import APIRouter
from .webhooks.stripe import stripe_webhook
from .webhooks.paypal import paypal_webhook
# from .webhooks.coinbase import coinbase_webhook

from .views.status import retrieve_status
from .views.stripe import create_stripe_customer
from .views.paypal import create_paypal_customer
# from .views.coinbase import create_coinbase_customer


router = APIRouter()

router.add_api_route("/subscribe/status", retrieve_status, methods=["POST"])
router.add_api_route("/subscribe/stripe", create_stripe_customer, methods=["POST"])
router.add_api_route("/subscribe/paypal", create_paypal_customer, methods=["POST"])
router.add_api_route("/webhooks/stripe", stripe_webhook, methods=["POST"])
router.add_api_route("/webhooks/paypal", paypal_webhook, methods=["POST"])
