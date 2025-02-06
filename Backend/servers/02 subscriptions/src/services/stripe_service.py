import stripe
from dotenv import load_dotenv

load_dotenv()

class StripeService:
    def __init__(self):
        stripe.api_key = os.getenv("STRIPE_SECRET_KEY")

    def get_stripe_invoice(self, invoice_id: str):
        # try redis for invoice id and return if result
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

    def get_stripe_invoices_data(customer_id: str, limit=10):
        try:
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

    def create_stripe_purchase_checkout_session(user_id: str, price: int, currency: str = "usd"):
        """
        Create a Stripe Checkout Session for a one-time payment.
        """
        try:
            session = stripe.checkout.Session.create(
                payment_method_types=["card", "apple_pay", "google_pay"],
                customer_email=f"user_{user_id}@example.com",
                line_items=[{
                    "price_data": {
                        "currency": currency,
                        "product_data": {"name": f"{Description}"},
                        "unit_amount": price * 100,  # Convert to cents
                    },
                    "quantity": 1,
                }],
                mode="payment",
                success_url=f"{STRIPE_SUCCESS_URL}?session_id={{CHECKOUT_SESSION_ID}}",
                cancel_url=STRIPE_CANCEL_URL,
            )
            return {"checkout_url": session.url}
        
        except stripe.error.StripeError as e:
            return {"error": str(e)}