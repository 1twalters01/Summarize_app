from fastapi import APIRouter
from .subscriptions import router as subscription_router
from .payments import router as payment_router
from .invoices import router as invoice_router
from .discounts import router as discount_router
from .refunds import router as refund_router
from .settings import router as settings_router
from .webhooks import router as webhook_router


router = APIRouter()
router.include_router(subscription_router, prefix="/subscriptions", tags=["Subscriptions"])
router.include_router(payment_router, prefix="/payments", tags=["One-Time Purchases"])
router.include_router(invoice_router, prefix="/invoices", tags=["Invoices"])
router.include_router(discount_router, prefix="/discounts", tags=["Discounts"])
router.include_router(refund_router, prefix="/refunds", tags=["Refunds"])
router.include_router(settings_router, prefix="/settings", tags=["Settings"])
router.include_router(webhook_router, prefix="/webhooks", tags=["Webhooks"])