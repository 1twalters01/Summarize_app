from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated
from src.views.billing.generate import generate_invoice
from src.views.billing.get import get_invoices
from src.views.billing.payments import retry_failed_payment

router = APIRouter()

router.add_api_route(
    "/billing/get-invoices",
    get_invoices,
    methods=["GET"],
    dependencies=[Depends(is_authenticated)],
)

router.add_api_route(
    "/billing/invoice",
    generate_invoice_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated)],
)
