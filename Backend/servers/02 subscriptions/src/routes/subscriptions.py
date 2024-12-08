from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated
from src.views.subscriptions.status import retrieve_status as retrieve_status_view
from src.views.subscriptions.stripe import (
    create_stripe_customer as create_stripe_customer_view,
)
from src.views.subscriptions.paypal import (
    create_paypal_customer as create_paypal_customer_view,
)

router = APIRouter()

router.add_api_route(
    "/subscription/status",
    retrieve_status_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)
router.add_api_route(
    "/subscription/stripe",
    create_stripe_customer_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)
router.add_api_route(
    "/subscription/paypal",
    create_paypal_customer_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)
