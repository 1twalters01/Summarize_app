from fastapi import APIRouter, Depends

from src.middleware.authentication import is_admin, is_authenticated
from src.views.books.get import get_books

from src.views.books.create.request import (
    post_request_book_creation,
    post_request_book_creation_confirmation,
)
from src.views.books.create.force import (
    post_force_book_creation,
    post_force_book_creation_confirmation,
)
from src.views.books.create.manual import (
    post_force_book_creation_manually,
    post_force_book_creation_manually_confirmation,
)

from src.views.books.modify.request import (
    post_request_book_modification,
    post_request_book_modification_confirmation,
)
from src.views.books.modify.force import (
    post_force_book_modification,
    post_force_book_modification_confirmation,
)

from src.views.books.delete.request import (
    post_request_book_deletion,
    post_request_book_deletion_confirmation,
)
from src.views.books.delete.force import (
    post_force_book_deletion,
    post_force_book_deletion_confirmation,
)

router = APIRouter()

# Get
router.add_api_route("/book", get_books, methods=["GET"])

# Create
router.add_api_route(
    "/book/creation/request", post_request_book_creation, methods=["POST"]
)
router.add_api_route(
    "/book/creation/request/confirmation",
    post_request_book_creation_confirmation,
    methods=["POST"],
)
router.add_api_route(
    "/book/creation/force",
    post_force_book_creation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/book/creation/force/confirmation",
    post_force_book_creation_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/book/creation/force/manually",
    post_force_book_creation_manually,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/book/creation/force/manually/confirmation",
    post_force_book_creation_manually_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)

# Modify
router.add_api_route(
    "/book/modification/request", post_request_book_modification, methods=["POST"]
)
router.add_api_route(
    "/book/modification/request/confirmation",
    post_request_book_modification_confirmation,
    methods=["POST"],
)
router.add_api_route(
    "/book/modification/force",
    post_force_book_modification,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/book/modification/force/confirmation",
    post_force_book_modification_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)

# Delete
router.add_api_route(
    "/book/deletion/request", post_request_book_deletion, methods=["POST"]
)
router.add_api_route(
    "/book/deletion/request/confirmation",
    post_request_book_deletion_confirmation,
    methods=["POST"],
)
router.add_api_route(
    "/book/deletion/force",
    post_force_book_deletion,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/book/deletion/force/confirmation",
    post_force_book_deletion_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
