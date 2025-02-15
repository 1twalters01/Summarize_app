from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated_middleware
from src.views.payments.payment import create_checkout_session_view

# Requires payment type enum
router.add_api_route(
    "/one-time-payment",
    create_checkout_session_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated_middleware)],
)
