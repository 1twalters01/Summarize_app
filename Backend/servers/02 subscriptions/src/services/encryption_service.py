from cryptography.fernet import Fernet
from dotenv import load_dotenv
import os

load_dotenv()

class EncryptionService:
    key: str = str(os.getenv("ENCRYPTION_KEY"))

    def __init__(self):
        pass
        # __key = os.getenv("ENCRYPTION_KEY")
        # if not __key:
        #     raise Exception("Invalid key")

    def encrypt(self, token):
        fernet = Fernet(self.key)
        encoded_token = token.encode("ascii")
        encrypted_token = fernet.encrypt(encoded_token).decode("ascii")
        formatted_token = encrypted_token.lstrip("b'").rstrip("'").replace("=", "%")
        return formatted_token

    def decrypt(self, token):
        fernet = Fernet(self.key)
        formatted_token = token.replace("%", "=").encode("ascii")
        decrypted_token = fernet.decrypt(formatted_token).decode("ascii")
        return decrypted_token
