from fastapi import APIRouter

from .views.get import get_books

from .views.create.request import post_request_book_creation, post_request_book_creation_confirmation
from .views.create.force import post_force_book_creation, post_force_book_creation_confirmation
from .views.create.manual import post_force_book_creation_manually, post_force_book_creation_manually_confirmation

from .views.modify.request import post_request_book_modification, post_request_book_modification_confirmation
from .views.modify.force import post_force_book_modification, post_force_book_modification_confirmation

from .views.delete.request import post_request_book_deletion, post_request_book_deletion_confirmation
from .views.delete.force import post_force_book_deletion, post_force_book_deletion_confirmation

router = APIRouter()

# Get
router.add_api_route("/book", get_books, methods=["GET"])

# Create
router.add_api_route("/book/creation/request", post_request_book_creation, methods=["POST"])
router.add_api_route("/book/creation/request/confirmation", post_request_book_creation_confirmation, methods=["POST"])
router.add_api_route("/book/creation/force", post_force_book_creation, methods=["POST"])
router.add_api_route("/book/creation/force/confirmation", post_force_book_creation_confirmation, methods=["POST"])
router.add_api_route("/book/creation/force/manually", post_force_book_creation_manually, methods=["POST"])
router.add_api_route("/book/creation/force/manually/confirmation", post_force_book_creation_manually_confirmation, methods=["POST"])

# Modify
router.add_api_route("/book/modification/request", post_request_book_modification, methods=["POST"])
router.add_api_route("/book/modification/request/confirmation", post_request_book_modification_confirmation, methods=["POST"])
router.add_api_route("/book/modification/force", post_force_book_modification, methods=["POST"])
router.add_api_route("/book/modification/force/confirmation", post_force_book_modification_confirmation, methods=["POST"])

# Delete
router.add_api_route("/book/deletion/request", post_request_book_deletion, methods=["POST"])
router.add_api_route("/book/deletion/request/confirmation", post_request_book_deletion_confirmation, methods=["POST"])
router.add_api_route("/book/deletion/force", post_force_book_deletion, methods=["POST"])
router.add_api_route("/book/deletion/force/confirmation", post_force_book_deletion_confirmation, methods=["POST"])

