from fastapi import APIRouter

from .views.premium import get_premium_status

router = APIRouter()

router.add_api_route("/settings/subscription", get_premium_status, methods=["POST"])

