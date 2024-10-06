from fastapi import APIRouter
from .views.create import post_request_book_creation, post_force_book_creation, post_force_book_creation_manually
from .views.modify import post_request_book_modification, post_force_book_modification
from .views.delete import post_request_book_deletion, post_force_book_deletion

router = APIRouter()

router.add_api_route("/book/request-creation", post_request_book_creation, methods=["POST"])
router.add_api_route("/book/request-modification", post_request_book_modification, methods=["POST"])
router.add_api_route("/book/request-deletion", post_request_book_deletion, methods=["POST"])
router.add_api_route("/book/force-creation", post_force_book_creation, methods=["POST"])
router.add_api_route("/book/force-creation/manually", post_force_book_creation_manually, methods=["POST"])
router.add_api_route("/book/force-modification", post_force_book_modification, methods=["POST"])
router.add_api_route("/book/force-deletion", post_force_book_deletion, methods=["POST"])

# Request new Book to be added [POST]
# Request Book information to be modified [POST]
# Request Book information to be deleted [POST]
# Admin add new Book [POST]
# Admin add new Book manually [POST]
# Admin modify Book information [POST]
# Admin delete Book information [POST]
# Admin view change requests [GET]
# Admin accept change requests (with posible modifications made) [POST]