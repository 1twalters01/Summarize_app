from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated
from src.views.purchases.pay import create_checkout_session_view

# Requires payment type enum
router.add_api_route(
    "/one-time-payment",
    create_checkout_session_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)
