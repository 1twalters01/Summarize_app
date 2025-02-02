from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated
from src.views.purchases.stripe import (
    create_checkout_session as create_stripe_purchase_view,
    retrieve_invoice as retrieve_stripe_invoice_view,
)
from src.views.purchases.paypal import (
    create_checkout_session as create_paypal_purchase_view,
    retrieve_invoice as retrieve_paypal_invoice_view,
)

router = APIRouter()

router.add_api_route(
    "/purchase/stripe/checkout",
    create_stripe_purchase_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)
router.add_api_route(
    "/purchase/stripe/invoice",
    retrieve_stripe_invoice_view,
    methods=["GET"],
    dependencies=[Depends(is_authenticated)],
)

router.add_api_route(
    "/purchase/paypal/checkout",
    create_paypal_purchase_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)
router.add_api_route(
    "/purchase/paypal/invoice",
    retrieve_paypal_invoice_view,
    methods=["GET"],
    dependencies=[Depends(is_authenticated)],
)
