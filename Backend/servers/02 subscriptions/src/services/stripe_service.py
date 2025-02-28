import stripe
from dotenv import load_dotenv

load_dotenv()

class StripeService:
    def __init__(self):
        stripe.api_key = os.getenv("STRIPE_SECRET_KEY")

    def get_stripe_invoice(self, invoice_id: str):
        # try redis for invoice id and return if result

        # else:
        try:
            invoice = stripe.Invoice.retrieve(invoice_id)
            return {
                "id": invoice.id,
                "status": invoice.status,
                "amount_due": invoice.amount_due / 100,
                "currency": invoice.currency,
                "created": invoice.created,
                "pdf": invoice.hosted_invoice_url
            }
        except stripe.error.StripeError as e:
            return None