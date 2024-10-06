from fastapi import APIRouter
from .views.get import get_change_requests, get_single_change_request
from .views.modify import post_change_request_modification
from .views.delete.single import post_change_request_deletion, post_change_request_deletion_confirmation
from .views.delete.multiple import post_change_requests_deletion, post_change_requests_deletion_confirmation

router = APIRouter()

router.add_api_route("/change-requests", get_change_requests, methods=["GET"])
router.add_api_route("/change-request/{:id}", get_single_change_request, methods=["GET"])
router.add_api_route("/change-request/modify/{:id}", post_change_request_modification, methods=["POST"])
router.add_api_route("/change-request/delete/{:id}", post_change_request_deletion, methods=["POST"])
router.add_api_route("/change-request/delete/{:id}/confirmation", post_change_request_deletion_confirmation, methods=["POST"])
router.add_api_route("/change-request/accept/{:id}", post_accept_change_request, methods=["POST"])
router.add_api_route("/change-requests/delete", post_delete_change_requests, methods=["POST"])
router.add_api_route("/change-requests/delete/confirmation", post_delete_change_requests_confirmation, methods=["POST"])


# Admin view change requests [GET]
# Admin view specific change request [GET]
# Admin modify specific change request [GET]
# Admin delete specific change request [POST]
# Admin delete specific change request confirmation [POST]
# Admin accept change request (with posible modifications made) [POST]
# Admin delete multiple change requests [POST]
# Admin delete multiple change requests confirmation [POST]