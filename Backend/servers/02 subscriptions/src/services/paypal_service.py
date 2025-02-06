import requests
from dotenv import load_dotenv
import os

load_dotenv()

PAYPAL_API_BASE = "https://api-m.sandbox.paypal.com/"

class PayPalSubscriptionService:
    client_id: str
    secret_id: str
    access_token: str | None

    def __init__(self):
        client_id = os.getenv("PAYPAL_CLIENT_ID")
        if client_id == None:
            raise Error("No paypal client id found")

        secret_id = os.getenv("PAYPAL_SECRET_KEY")
        if secret_id == None:
            raise Error("No paypal secret id found")

        self.client_id = client_id
        self.secret_id = secret_id
        if not self.access_token:
            self.access_token = self._get_access_token()

    def _get_access_token(self) -> str | None:
        """Fetch and store the PayPal access token."""
        url = f"{PAYPAL_API_BASE}/v1/oauth2/token"
        headers = {"Content-Type": "application/x-www-form-urlencoded"}
        data = {"grant_type": "client_credentials"}
        user = (self.client_id, self.secret_id)

        response = requests.post(url, auth=user, headers=headers, data=data)
        response.raise_for_status()
        return response.json()["access_token"]

    def _get_headers(self):
        """Authorization header."""
        return {
            "Content-Type": "application/json",
            "Authorization": f"Bearer {self.access_token}",
        }

    def get_paypal_invoice(invoice_id: str):
        url = f"{PAYPAL_API_BASE}/v2/invoicing/invoices/{invoice_id}"
        response = requests.get(url, headers=self._get_headers())

        if response.status_code == 200:
            data = response.json()
            return {
                "id": data.get("id"),
                "status": data.get("status"),
                "amount_due": data.get("amount", {}).get("value"),
                "currency": data.get("amount", {}).get("currency_code"),
                "created": data.get("create_time"),
                "pdf": data.get("links", [])[0]["href"]
            }
        return None

    def get_paypal_invoices_data(customer_id: str, limit=10):
        """
        Fetch all PayPal invoices associated with a given customer.
        """
        url = f"{PAYPAL_API_BASE}/v2/invoicing/invoices?recipient_email={customer_id}"
        response = requests.get(url, headers=self._get_headers())

        if response.status_code == 200:
            data = response.json()
            invoice_list = []
        
            for invoice in data.get("invoices", []):
                invoice_element = {
                    "id": invoice.get("id"),
                    "status": invoice.get("status"),
                    "amount_due": invoice.get("amount", {}).get("value"),
                    "currency": invoice.get("amount", {}).get("currency_code"),
                    "created": invoice.get("create_time"),
                }
                invoice_list.append(invoice_element)

                pdf_url = next((link["href"] for link in invoice.get("links", []) if link["rel"] == "self"), None)
                redis_key = f"invoice_id:{invoice_id}"
                redis_client.set(redis_key, (pdf_url, invoice_element), ex=expiry_in_seconds)
            return invoices

        return None


    def show_sub_details(self, subscription_id):
        url = f"v1/billing/subscriptions/{subscription_id}"
        response = requests.get(url, headers=self._get_headers())
        response.raise_for_status()
        return response.json()
        
    def cancel_sub(self, subscription_id, reason="Not satisfied with the service"):
        url = f"{PAYPAL_API_BASE}/v1/billing/subscriptions/{subscription_id}/cancel"
        data = {"reason": reason}
        response = requests.post(url, headers=self._get_headers(), json=data)
        response.raise_for_status()
        return response.json()

    def suspend_sub(self, subscription_id, reason="Not satisfied with the service"):
        url = f"{PAYPAL_API_BASE}/v1/billing/subscriptions/{subscription_id}/suspend"
        data = {"reason": reason}
        response = requests.post(url, headers=self._get_headers(), json=data)
        response.raise_for_status()
        print(response.json())  # Remove line after checking what this actually is
        return {"success": True}

    def activate_sub(self, subscription_id, reason="Not satisfied with the service"):
        url = f"{PAYPAL_API_BASE}/v1/billing/subscriptions/{subscription_id}/activate"
        data = {"reason": reason}
        response = requests.post(url, headers=self._get_headers(), json=data)
        response.raise_for_status()
        return {"success": True}

    def create_paypal_purchase_order(user_id: str, price: float, currency: str = "GBP"):
        """Create a PayPal order for a one-time payment."""
        url = f"{PAYPAL_API_BASE}/v2/checkout/orders"
        data = {
            "intent": "CAPTURE",
            "purchase_units": [{
                "amount": {"currency_code": currency, "value": str(price)},
                "description": f"{Description}",
            }],
            "redirect_urls": {
                "return_url": f"{SUCCESS_URL}",
                "cancel_url": f"{CANCEL_URL}"
            }
        }

        response = requests.post(url, json=data, headers=self._get_headers())
        if response.status_code == 201:
            order = response.json()
            return {"checkout_url": order["links"][1]["href"]}
        
        return {"error": response.json()}
