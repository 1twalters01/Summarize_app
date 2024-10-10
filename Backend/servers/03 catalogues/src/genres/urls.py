from fastapi import APIRouter
from .views.create_genre import post_request_genre_creation, post_force_genre_creation, post_force_genre_creation_manually
from .views.modify_genre import post_request_genre_modification, post_force_genre_modification
from .views.delete_genre import post_request_genre_deletion, post_force_genre_deletion

router = APIRouter()

router.add_api_route("/genre/request-creation", post_request_genre_creation, methods=["POST"])
router.add_api_route("/genre/request-modification", post_request_genre_modification, methods=["POST"])
router.add_api_route("/genre/request-deletion", post_request_genre_deletion, methods=["POST"])
router.add_api_route("/genre/force-creation", post_force_genre_creation, methods=["POST"])
router.add_api_route("/genre/force-creation/manually", post_force_genre_creation_manually, methods=["POST"])
router.add_api_route("/genre/force-modification", post_force_genre_modification, methods=["POST"])
router.add_api_route("/genre/force-deletion", post_force_genre_deletion, methods=["POST"])