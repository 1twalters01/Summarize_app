from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated
from src.views.subscriptions.status import retrieve_status
from src.views.subscriptions.stripe import create_stripe_customer
from src.views.subscriptions.paypal import create_paypal_customer
# from .views.coinbase import create_coinbase_customer

router = APIRouter()

router.add_api_route(
    "/subscribe/status",
    retrieve_status,
    methods=["POST"],
    dependencies = [Depends(is_authenticated)]
)
router.add_api_route(
    "/subscribe/stripe",
    create_stripe_customer,
    methods=["POST"],
    dependencies = [Depends(is_authenticated)]
)
router.add_api_route(
    "/subscribe/paypal",
    create_paypal_customer,
    methods=["POST"],
    dependencies = [Depends(is_authenticated)]
)
