from fastapi import APIRouter
from .views.optional_authentication import get_any_auth, post_any_auth
from .views.requires_authentication import get_only_auth, post_only_auth
from .views.no_authentication import get_not_auth, post_not_auth

router = APIRouter()

router.add_api_route("/ping/any_auth", get_any_auth, methods=["GET"])
router.add_api_route("/ping/any_auth", post_any_auth, methods=["POST"])
router.add_api_route("/ping/only_auth", get_only_auth, methods=["GET"])
router.add_api_route("/ping/only_auth", post_only_auth, methods=["POST"])
router.add_api_route("/ping/not_auth", get_not_auth, methods=["GET"])
router.add_api_route("/ping/not_auth", post_not_auth, methods=["POST"])
