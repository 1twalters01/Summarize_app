import requests
from dotenv import load_dotenv
import os

load_dotenv()


class PayPalSubscriptionService:
    client_id: str
    secret_id: str
    access_token: str | None

    def __init__(self):
        client_id = os.getenv("PAYPAL_CLIENT_ID")
        secret_id = os.getenv("PAYPAL_SECRET_KEY")

        if client_id == None or secret_id == None:
            return None
            # raise TypeError

        self.client_id = client_id
        self.secret_id = secret_id
        self.access_token = self._get_access_token()

    def _get_access_token(self) -> str | None:
        """Fetch and store the PayPal access token."""
        url = "https://api-m.sandbox.paypal.com/v1/oauth2/token"
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

    def show_sub_details(self, subscription_id):
        url = f"https://api-m.sandbox.paypal.com/v1/billing/subscriptions/{subscription_id}"
        response = requests.get(url, headers=self._get_headers())
        response.raise_for_status()
        return response.json()

    def cancel_sub(self, subscription_id, reason="Not satisfied with the service"):
        url = f"https://api-m.sandbox.paypal.com/v1/billing/subscriptions/{subscription_id}/cancel"
        data = {"reason": reason}
        response = requests.post(url, headers=self._get_headers(), json=data)
        response.raise_for_status()
        return response.json()

    def suspend_sub(self, subscription_id, reason="Not satisfied with the service"):
        url = f"https://api-m.sandbox.paypal.com/v1/billing/subscriptions/{subscription_id}/suspend"
        data = {"reason": reason}
        response = requests.post(url, headers=self._get_headers(), json=data)
        response.raise_for_status()
        print(response.json())  # Remove line after checking what this actually is
        return {"success": True}

    def activate_sub(self, subscription_id, reason="Not satisfied with the service"):
        url = f"https://api-m.sandbox.paypal.com/v1/billing/subscriptions/{subscription_id}/activate"
        data = {"reason": reason}
        response = requests.post(url, headers=self._get_headers(), json=data)
        response.raise_for_status()
        return {"success": True}


# subscription_id = 'I-W0F4P2H7MDNJ'
# print(show_sub_details(subscription_id))
# suspend_sub(get_access_token(), subscription_id)
# cancel_sub(subscription_id)
# activate_sub(get_access_token(), subscription_id)
