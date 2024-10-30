from fastapi import APIRouter

from src.views.settings.premium import get_premium_status

router = APIRouter()

router.add_api_route("/settings/subscription", get_premium_status, methods=["POST"])

