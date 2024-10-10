from fastapi import APIRouter
from .views.create import post_request_author_creation, post_force_author_creation, post_force_author_creation_manually
from .views.modify import post_request_author_modification, post_force_author_modification
from .views.delete import post_request_author_deletion, post_force_author_deletion

router = APIRouter()

router.add_api_route("/author/request-creation", post_request_author_creation, methods=["POST"])
router.add_api_route("/author/force-creation", post_force_author_creation, methods=["POST"])
router.add_api_route("/author/request-modification", post_request_author_modification, methods=["POST"])
router.add_api_route("/author/force-creation/manually", post_force_author_creation_manually, methods=["POST"])
router.add_api_route("/author/force-modification", post_force_author_modification, methods=["POST"])
router.add_api_route("/author/request-deletion", post_request_author_deletion, methods=["POST"])
router.add_api_route("/author/force-deletion", post_force_author_deletion, methods=["POST"])