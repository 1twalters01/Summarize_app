from fastapi import APIRouter, Depends

from src.middleware.authentication import is_admin, is_authenticated
from src.views.genres.get import get_genres

from src.views.genres.create.request import (
    post_request_genre_creation,
    post_request_genre_creation_confirmation,
)
from src.views.genres.create.force import (
    post_force_genre_creation,
    post_force_genre_creation_confirmation,
)
from src.views.genres.create.manual import (
    post_force_genre_creation_manually,
    post_force_genre_creation_manually_confirmation,
)

from src.views.genres.modify.request import (
    post_request_genre_modification,
    post_request_genre_modification_confirmation,
)
from src.views.genres.modify.force import (
    post_force_genre_modification,
    post_force_genre_modification_confirmation,
)

from src.views.genres.delete.request import (
    post_request_genre_deletion,
    post_request_genre_deletion_confirmation,
)
from src.views.genres.delete.force import (
    post_force_genre_deletion,
    post_force_genre_deletion_confirmation,
)

router = APIRouter()

# Get
router.add_api_route("/genre", get_genres, methods=["GET"])

# Create
router.add_api_route(
    "/genre/creation/request",
    post_request_genre_creation,
    methods=["POST"],
)
router.add_api_route(
    "/genre/creation/request/confirmation",
    post_request_genre_creation_confirmation,
    methods=["POST"],
)
router.add_api_route(
    "/genre/creation/force",
    post_force_genre_creation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/genre/creation/force/confirmation",
    post_force_genre_creation_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/genre/creation/force/manually",
    post_force_genre_creation_manually,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/genre/creation/force/manually/confirmation",
    post_force_genre_creation_manually_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)

# Modify
router.add_api_route(
    "/genre/modification/request", post_request_genre_modification, methods=["POST"]
)
router.add_api_route(
    "/genre/modification/request/confirmation",
    post_request_genre_modification_confirmation,
    methods=["POST"],
)
router.add_api_route(
    "/genre/modification/force",
    post_force_genre_modification,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/genre/modification/force/confirmation",
    post_force_genre_modification_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)

# Delete
router.add_api_route(
    "/genre/deletion/request",
    post_request_genre_deletion,
    methods=["POST"],
)
router.add_api_route(
    "/genre/deletion/request/confirmation",
    post_request_genre_deletion_confirmation,
    methods=["POST"],
)
router.add_api_route(
    "/genre/deletion/force",
    post_force_genre_deletion,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/genre/deletion/force/confirmation",
    post_force_genre_deletion_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
