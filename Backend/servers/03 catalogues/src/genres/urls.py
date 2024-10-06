from fastapi import APIRouter
from .views.optional_authentication import get_any_auth, post_any_auth

router = APIRouter()

router.add_api_route("/ping/any_auth", get_any_auth, methods=["GET"])
router.add_api_route("/ping/any_auth", post_any_auth, methods=["POST"])