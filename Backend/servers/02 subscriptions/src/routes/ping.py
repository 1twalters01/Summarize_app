from fastapi import APIRouter, Depends
from src.views.ping.subscription_status import get_subscription_status_view

router.add_api_route(
    "/ping/get-subscription-status",
    get_subscription_status_view,
    methods=["GET"],
)