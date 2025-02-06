from fastapi import Request, status
from src.services import stripe_service, paypal_service

async def get_invoices_view(request: Request):
    user_uuid = request.state.user_uuid
    # get customer_id

    stripe_invoice_data = stripe_service.get_stripe_invoice_data(customer_id)
    paypal_invoice_data = paypal_service.get_paypal_invoice_data(user_uuid)
    invoice_data = sorted(
        stripe_invoice_data + paypal_invoice_data,
        key=lambda x: x[1],
    )

    if not invoices:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="No invoices found",
        )

    return {"invoices": invoice_data}
