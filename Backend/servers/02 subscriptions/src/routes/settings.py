from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated_middleware
from src.views.settings.status import retrieve_status_view

router.add_api_route(
    "/settings/status",
    retrieve_status_view,
    methods=["GET"],
    dependencies=[Depends(is_authenticated_middleware)],
)
