from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated_middleware
from src.views.billing.discounts import add_discount_view
from src.views.billing.generate_invoice import generate_invoice_view
from src.views.billing.get_invoices import get_invoices_view
from src.views.billing.retry import retry_failed_payment_view

router = APIRouter()

router.add_api_route(
    "/billing/add-discount",
    add_discount_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated_middleware)],
)

router.add_api_route(
    "/billing/get-invoices",
    get_invoices_view,
    methods=["GET"],
    dependencies=[Depends(is_authenticated_middleware)],
)

router.add_api_route(
    "/billing/invoice",
    generate_invoice_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated_middleware)],
)

router.add_api_route(
    "/billing/retry-payment",
    retry_failed_payment_view,
    methods=["POST"],
    dependencies=[Depends(is_authenticated_middleware)],
)