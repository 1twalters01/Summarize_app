from fastapi import APIRouter
from .views.create_publisher import post_request_publisher_creation, post_force_publisher_creation, post_force_publisher_creation_manually
from .views.modify_publisher import post_request_publisher_modification, post_force_publisher_modification
from .views.delete_publisher import post_request_publisher_deletion, post_force_publisher_deletion

router = APIRouter()

router.add_api_route("/publisher/request-creation", post_request_publisher_creation, methods=["POST"])
router.add_api_route("/publisher/request-modification", post_request_publisher_modification, methods=["POST"])
router.add_api_route("/publisher/request-deletion", post_request_publisher_deletion, methods=["POST"])
router.add_api_route("/publisher/force-creation", post_force_publisher_creation, methods=["POST"])
router.add_api_route("/publisher/force-creation/manually", post_force_publisher_creation_manually, methods=["POST"])
router.add_api_route("/publisher/force-modification", post_force_publisher_modification, methods=["POST"])
router.add_api_route("/publisher/force-deletion", post_force_publisher_deletion, methods=["POST"])
