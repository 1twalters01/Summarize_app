from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated
from src.views.purchases.stripe import create_checkout_session as stripe_checkout
from src.views.purchases.paypal import create_checkout_session as paypal_checkout

router.add_api_route(
    "/purchase/stripe",
    stripe_checkout,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)

router.add_api_route(
    "/purchase/paypal",
    paypal_checkout,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)

router.add_api_route(
    "/purchase/crypto",
    paypal_checkout,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)

router.add_api_route(
    "/purchases/retry-payment",
    retry_failed_payment_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)