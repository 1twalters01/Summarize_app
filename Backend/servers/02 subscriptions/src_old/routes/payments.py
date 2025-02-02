from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated

router = APIRouter()

router.add_api_route(
    "/payment/invoice/{id}",
    retrieve_stripe_invoice_view,
    methods=["GET"],
    dependencies=[Depends(is_authenticated)],
)