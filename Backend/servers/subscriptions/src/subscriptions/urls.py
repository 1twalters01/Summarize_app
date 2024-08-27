from fastapi import APIRouter
from .webhooks.stripe import stripe_webhook
from .webhooks.paypal import paypal_webhook
from .webhooks.coinbase import coinbase_webhook
# from .webhooks import stripe.stripe_webhook, paypal.paypal_webhook, coinbase.coinbase_webhook
from .views.stripe import NewStripeCustomer
from .views.paypal import NewPaypalCustomer
from .views.coinbase import NewCoinbaseCustomer
# from .vews import stripe.NewStripeCustomer, paypal.NewPaypalCustomer, coinbase.NewCoinbaseCustomer

router = APIRouter()

router.add_api_route("/webhooks/stripe", stripe_webhook, methods=["POST"])
router.add_api_route("/webhooks/paypal", paypal_webhook, methods=["POST"])
# router.add_api_route("/webhooks/coinbase", coinbase_webhook, methods=["POST"])
router.add_api_route("/subscribe/stripe", stripe_webhook, methods=["POST"])
router.add_api_route("/subscribe/paypal", paypal_webhook, methods=["POST"])
# router.add_api_route("/subscriptions/coinbase", coinbase_webhook, methods=["POST"])
