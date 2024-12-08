from fastapi import APIRouter, Depends

from src.middleware.authentication import is_admin, is_authenticated
from src.views.publishers.get import get_publishers

from src.views.publishers.create.request import (
    post_request_publisher_creation,
    post_request_publisher_creation_confirmation,
)
from src.views.publishers.create.force import (
    post_force_publisher_creation,
    post_force_publisher_creation_confirmation,
)
from src.views.publishers.create.manual import (
    post_force_publisher_creation_manually,
    post_force_publisher_creation_manually_confirmation,
)

from src.views.publishers.modify.request import (
    post_request_publisher_modification,
    post_request_publisher_modification_confirmation,
)
from src.views.publishers.modify.force import (
    post_force_publisher_modification,
    post_force_publisher_modification_confirmation,
)

from src.views.publishers.delete.request import (
    post_request_publisher_deletion,
    post_request_publisher_deletion_confirmation,
)
from src.views.publishers.delete.force import (
    post_force_publisher_deletion,
    post_force_publisher_deletion_confirmation,
)

router = APIRouter()

# Get
router.add_api_route("/publisher", get_publishers, methods=["GET"])

# Create
router.add_api_route(
    "/publisher/creation/request", post_request_publisher_creation, methods=["POST"]
)
router.add_api_route(
    "/publisher/creation/request/confirmation",
    post_request_publisher_creation_confirmation,
    methods=["POST"],
)
router.add_api_route(
    "/publisher/creation/force",
    post_force_publisher_creation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/publisher/creation/force/confirmation",
    post_force_publisher_creation_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/publisher/creation/force/manually",
    post_force_publisher_creation_manually,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/publisher/creation/force/manually/confirmation",
    post_force_publisher_creation_manually_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)

# Modify
router.add_api_route(
    "/publisher/modification/request",
    post_request_publisher_modification,
    methods=["POST"],
)
router.add_api_route(
    "/publisher/modification/request/confirmation",
    post_request_publisher_modification_confirmation,
    methods=["POST"],
)
router.add_api_route(
    "/publisher/modification/force",
    post_force_publisher_modification,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/publisher/modification/force/confirmation",
    post_force_publisher_modification_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)

# Delete
router.add_api_route(
    "/publisher/deletion/request", post_request_publisher_deletion, methods=["POST"]
)
router.add_api_route(
    "/publisher/deletion/request/confirmation",
    post_request_publisher_deletion_confirmation,
    methods=["POST"],
)
router.add_api_route(
    "/publisher/deletion/force",
    post_force_publisher_deletion,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/publisher/deletion/force/confirmation",
    post_force_publisher_deletion_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
