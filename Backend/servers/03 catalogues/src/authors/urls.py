from fastapi import APIRouter
from .views.create import post_request_author_creation, post_force_author_creation, post_force_author_creation_manually
from .views.modify import post_request_author_modification, post_force_author_modification
from .views.delete import post_request_author_deletion, post_force_author_deletion

router = APIRouter()

router.add_api_route("/author/request-creation", post_request_author_creation, methods=["POST"])
router.add_api_route("/author/request-modification", post_request_author_modification, methods=["POST"])
router.add_api_route("/author/request-deletion", post_request_author_deletion, methods=["POST"])
router.add_api_route("/author/force-creation", post_force_author_creation, methods=["POST"])
router.add_api_route("/author/force-creation/manually", post_force_author_creation_manually, methods=["POST"])
router.add_api_route("/author/force-modification", post_force_author_modification, methods=["POST"])
router.add_api_route("/author/force-deletion", post_force_author_deletion, methods=["POST"])

# Request new Author to be added [POST]
# Request Author information to be modified [POST]
# Request Author information to be deleted [POST]
# Admin add new Author [POST]
# Admin add new Author manually [POST]
# Admin modify Author information [POST]
# Admin delete Author information [POST]
# Admin view change requests [GET]
# Admin accept change requests (with posible modifications made) [POST]