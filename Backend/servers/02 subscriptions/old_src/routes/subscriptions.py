from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated
from src.views.subscriptions.status import retrieve_status_view
from src.views.subscriptions.plans import retrieve_plans_view
from src.views.subscriptions.cancellation import cancel_subscription_view
from src.views.subscriptions.create import create_subscription_view
from src.views.subscriptions.retry import retry_failed_payment_view

router.add_api_route(
    "/subscription/plans",
    retrieve_plans_view,
    methods=["GET"],
)

router.add_api_route(
    "/subscription/status",
    retrieve_status_view,
    methods=["GET"],
    dependencies=[Depends(is_authenticated)],
)

router.add_api_route(
    "/subscription/cancel",
    cancel_subscription_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)

router.add_api_route(
    "/subscription/create",
    create_subscription_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)

router.add_api_route(
    "/subscription/retry-payment",
    retry_failed_payment_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)