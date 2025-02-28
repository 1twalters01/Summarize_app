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

    def get_stripe_invoices_data(encrypted_customer_id: str|None, limit=10):
        if encrypted_customer_id == None:
            return None

        try:
            encryption_service = EncryptionService()
            customer_id = encryption_service.decrypt(encrypted_customer_id)
            invoices = stripe.Invoice.list(customer=customer_id, limit=limit)

            if not invoices.data:
                return []

            invoice_list = []
            for invoice in invoices.auto_paging_iter():
                invoice_element = {
                    "id": invoice.id,
                    "status": invoice.status,
                    "amount_due": invoice.amount_due,
                    "currency": invoice.currency.upper(),
                    "created": invoice.created,
                }
                invoice_list.append(invoice_element)

                pdf_url = invoice.hosted_invoice_url
                redis_key = f"invoice_id:{invoice_id}"
                redis_client.set(redis_key, (pdf_url, invoice_element), ex=expiry_in_seconds)
            return invoice_list

        except stripe.error.StripeError as e:
            return {"error": str(e)}