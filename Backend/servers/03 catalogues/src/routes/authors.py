from fastapi import APIRouter

from .views.authors.get import get_authors

from .views.create.request import post_request_author_creation, post_request_author_creation_confirmation
from .views.create.force import post_force_author_creation, post_force_author_creation_confirmation
from .views.create.manual import post_force_author_creation_manually, post_force_author_creation_manually_confirmation

from .views.modify.request import post_request_author_modification, post_request_author_modification_confirmation
from .views.modify.force import post_force_author_modification, post_force_author_modification_confirmation

from .views.delete.request import post_request_author_deletion, post_request_author_deletion_confirmation
from .views.delete.force import post_force_author_deletion, post_force_author_deletion_confirmation

router = APIRouter()

# Get
router.add_api_route("/author", get_authors, methods=["GET"])

# Create
router.add_api_route("/author/creation/request", post_request_author_creation, methods=["POST"])
router.add_api_route("/author/creation/request/confirmation", post_request_author_creation_confirmation, methods=["POST"])
router.add_api_route("/author/creation/force", post_force_author_creation, methods=["POST"])
router.add_api_route("/author/creation/force/confirmation", post_force_author_creation_confirmation, methods=["POST"])
router.add_api_route("/author/creation/force/manually", post_force_author_creation_manually, methods=["POST"])
router.add_api_route("/author/creation/force/manually/confirmation", post_force_author_creation_manually_confirmation, methods=["POST"])

# Modify
router.add_api_route("/author/modification/request", post_request_author_modification, methods=["POST"])
router.add_api_route("/author/modification/request/confirmation", post_request_author_modification_confirmation, methods=["POST"])
router.add_api_route("/author/modification/force", post_force_author_modification, methods=["POST"])
router.add_api_route("/author/modification/force/confirmation", post_force_author_modification_confirmation, methods=["POST"])

# Delete
router.add_api_route("/author/deletion/request", post_request_author_deletion, methods=["POST"])
router.add_api_route("/author/deletion/request/confirmation", post_request_author_deletion_confirmation, methods=["POST"])
router.add_api_route("/author/deletion/force", post_force_author_deletion, methods=["POST"])
router.add_api_route("/author/deletion/force/confirmation", post_force_author_deletion_confirmation, methods=["POST"])
