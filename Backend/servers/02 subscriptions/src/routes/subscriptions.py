from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated
from src.views.subscriptions.status import retrieve_status_view
from src.views.subscriptions.plans import retrieve_plans_view
from src.views.subscriptions.cancellation import cancel_subscription_view
from src.views.subscriptions.create_stripe import create_stripe_subscription_view
from src.views.subscriptions.create_paypal import create_paypal_subscription_view

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
    "/subscription/create/stripe",
    create_subscription_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)

router.add_api_route(
    "/subscription/create/paypal",
    create_subscription_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)