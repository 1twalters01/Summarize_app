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

# Request new Genre to be added [POST]
# Request Genre information to be modified [POST]
# Request Genre information to be deleted [POST]
# Admin add new Genre [POST]
# Admin add new Genre manually [POST]
# Admin modify Genre information [POST]
# Admin delete Genre information [POST]
# Admin view change requests [GET]
# Admin accept change requests (with posible modifications made) [POST]