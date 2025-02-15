from fastapi import APIRouter
from .subscriptions import router as subscription_router
from .purchases import router as purchases_router
from .invoices import router as invoices_router
from .webhooks import router as webhook_router
from .admin import router as admin_router

router = APIRouter()

router.include_router(invoices_router, prefix="/invoices", tags=["Invoices"])
router.include_router(subscription_router, prefix="/subscriptions", tags=["Subscriptions"])
router.include_router(purchases_router, prefix="/purchases", tags=["One-Time Purchases"])
router.include_router(webhook_router, prefix="/webhooks", tags=["Webhooks"])
