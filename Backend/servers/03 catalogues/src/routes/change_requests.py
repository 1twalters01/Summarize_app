from fastapi import APIRouter
from .views.get import get_change_requests, get_single_change_request
from .views.modify import post_change_request_modification
from .views.delete.single import post_change_request_deletion, post_change_request_deletion_confirmation
from .views.change_request import post_accept_change_request

router = APIRouter()

router.add_api_route("/change-requests", get_change_requests, methods=["GET"])
router.add_api_route("/change-request/{:id}", get_single_change_request, methods=["GET"])
router.add_api_route("/change-request/modify/{:id}", post_change_request_modification, methods=["POST"])
router.add_api_route("/change-request/delete", post_change_request_deletion, methods=["POST"])
router.add_api_route("/change-request/delete/confirmation", post_change_request_deletion_confirmation, methods=["POST"])
router.add_api_route("/change-request/accept/{:id}", post_accept_change_request, methods=["POST"])
