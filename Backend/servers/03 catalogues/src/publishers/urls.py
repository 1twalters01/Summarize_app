from fastapi import APIRouter

from .views.get import get_publishers

from .views.create.request import post_request_publisher_creation, post_request_publisher_creation_confirmation
from .views.create.force import post_force_publisher_creation, post_force_publisher_creation_confirmation
from .views.create.manual import post_force_publisher_creation_manually, post_force_publisher_creation_manually_confirmation

from .views.modify.request import post_request_publisher_modification, post_request_publisher_modification_confirmation
from .views.modify.force import post_force_publisher_modification, post_force_publisher_modification_confirmation

from .views.delete.request import post_request_publisher_deletion, post_request_publisher_deletion_confirmation
from .views.delete.force import post_force_publisher_deletion, post_force_publisher_deletion_confirmation

router = APIRouter()

# Get
router.add_api_route("/publisher", get_publishers, methods=["GET"])

# Create
router.add_api_route("/publisher/creation/request", post_request_publisher_creation, methods=["POST"])
router.add_api_route("/publisher/creation/request/confirmation", post_request_publisher_creation_confirmation, methods=["POST"])
router.add_api_route("/publisher/creation/force", post_force_publisher_creation, methods=["POST"])
router.add_api_route("/publisher/creation/force/confirmation", post_force_publisher_creation_confirmation, methods=["POST"])
router.add_api_route("/publisher/creation/force/manually", post_force_publisher_creation_manually, methods=["POST"])
router.add_api_route("/publisher/creation/force/manually/confirmation", post_force_publisher_creation_manually_confirmation, methods=["POST"])

# Modify
router.add_api_route("/publisher/modification/request", post_request_publisher_modification, methods=["POST"])
router.add_api_route("/publisher/modification/request/confirmation", post_request_publisher_modification_confirmation, methods=["POST"])
router.add_api_route("/publisher/modification/force", post_force_publisher_modification, methods=["POST"])
router.add_api_route("/publisher/modification/force/confirmation", post_force_publisher_modification_confirmation, methods=["POST"])

# Delete
router.add_api_route("/publisher/deletion/request", post_request_publisher_deletion, methods=["POST"])
router.add_api_route("/publisher/deletion/request/confirmation", post_request_publisher_deletion_confirmation, methods=["POST"])
router.add_api_route("/publisher/deletion/force", post_force_publisher_deletion, methods=["POST"])
router.add_api_route("/publisher/deletion/force/confirmation", post_force_publisher_deletion_confirmation, methods=["POST"])
