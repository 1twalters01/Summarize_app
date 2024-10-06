from fastapi import APIRouter
from .views.create_author import post_request_new_author, post_force_new_author
from .views.modify_author import post_request_author_modification, post_force_author_modification
from .views.delete_author import post_request_author_deletion, post_force_author_deletion

router = APIRouter()

router.add_api_route("/author/request-new-author", post_request_new_author, methods=["POST"])
router.add_api_route("/author/request-author-modification", post_request_author_modification, methods=["POST"])
router.add_api_route("/author/request-author-deletion", post_request_author_deletion, methods=["POST"])
router.add_api_route("/author/force-new-author", post_force_new_author, methods=["POST"])
router.add_api_route("/author/force-author-modification", post_force_author_modification, methods=["POST"])
router.add_api_route("/author/force-author-deletion", post_force_author_deletion, methods=["POST"])

# Request new Author to be added [POST]
# Request Author information to be modified [POST]
# Request Author information to be deleted [POST]
# Admin add new Author [POST]
# Admin modify Author information [POST]
# Admin delete Author information [POST]
# Admin view change requests [GET]
# Admin accept change requests (with posible modifications made) [POST]