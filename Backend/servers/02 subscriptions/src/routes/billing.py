from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated
from src.views.billing.invoices import generate_invoice_view
from src.views.billing.payments import retry_failed_payment_view

router = APIRouter()

router.add_api_route(
    "/billing/invoice",
    generate_invoice_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)

router.add_api_route(
    "/billing/retry-payment",
    retry_failed_payment_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)