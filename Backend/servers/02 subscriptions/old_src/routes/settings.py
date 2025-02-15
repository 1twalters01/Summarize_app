from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated
from src.views.settings.status import retrieve_status_view

router.add_api_route(
    "/subscription/status",
    retrieve_status_view,
    methods=["GET"],
    dependencies=[Depends(is_authenticated)],
)
