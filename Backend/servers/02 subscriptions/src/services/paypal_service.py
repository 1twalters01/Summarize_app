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

    def get_paypal_invoice(self, invoice_id: str):
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