from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated_middleware
from src.views.subscriptions.create import create_subscription_view
from src.views.subscriptions.cancellation import cancel_subscription_view

# Create subscription, requires payment type enum
router.add_api_route(
    "/subscription/create",
    create_subscription_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated_middleware)],
)

router.add_api_route(
    "/subscription/cancel",
    cancel_subscription_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated_middleware)],
)