from fastapi import APIRouter, Depends
from src.middleware.authentication import is_admin, is_authenticated
from src.views.change_requests.get import get_change_requests, get_single_change_request
from src.views.change_requests.modify import post_change_request_modification
from src.views.change_requests.delete import (
    post_change_request_deletion,
    post_change_request_deletion_confirmation,
)
from src.views.change_requests.change_request import post_accept_change_request

router = APIRouter()

router.add_api_route("/change-requests", get_change_requests, methods=["GET"])
router.add_api_route(
    "/change-request/{:id}",
    get_single_change_request,
    methods=["GET"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/change-request/modify/{:id}",
    post_change_request_modification,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/change-request/delete",
    post_change_request_deletion,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/change-request/delete/confirmation",
    post_change_request_deletion_confirmation,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
router.add_api_route(
    "/change-request/accept/{:id}",
    post_accept_change_request,
    methods=["POST"],
    dependencies=[Depends(is_authenticated), Depends(is_admin)],
)
