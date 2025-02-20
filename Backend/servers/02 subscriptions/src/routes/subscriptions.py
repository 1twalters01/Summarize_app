from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated_middleware
from src.views.subscriptions.create_urls import create_subscription_view
from src.views.subscriptions.cancellation import cancel_subscription_view
from src.views.subscriptions.pause import pause_subscription_view
from src.views.subscriptions.resume import resume_subscription_view
from src.views.subscriptions.change import change_subscription_view

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

router.add_api_route(
    "/subscription/pause",
    pause_subscription_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated_middleware)],
)

router.add_api_route(
    "/subscription/resume",
    resume_subscription_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated_middleware)],
)

router.add_api_route(
    "/subscription/change-plan",
    change_subscription_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated_middleware)],
)