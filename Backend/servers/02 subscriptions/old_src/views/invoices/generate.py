from fastapi import Request, status
from src.services import user_service, stripe_service, paypal_service

async def generate_invoice_view(request: Request):
    user_uuid = request.state.user_uuid
    payment_provider = user_service.get_payment_provider_for_uuid(user_uuid)

    if payment_provider == "stripe":
        invoice = stripe_service.get_stripe_invoice(user_uuid, invoice_id)
    elif payment_provider == "paypal":
        invoice = paypal_service.get_paypal_invoice(user_uuid, invoice_id)
    else:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Invalid payment provider",
        )

    if not invoice:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Invoice not found",
        )

    return {"invoice": invoice}