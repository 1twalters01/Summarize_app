from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated

from src.views.settings.premium import get_premium_status

router = APIRouter()

router.add_api_route(
    "/settings/subscription",
    get_premium_status,
    methods=["POST"],
    dependencies = [Depends(is_authenticated)]
)

