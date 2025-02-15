from src.services.encryption_service import EncryptionService
import string
import random


def test_encryption_service():
    length = 10
    code = ''.join(random.choices(string.ascii_uppercase + string.digits, k=length))

    encryption_service = EncryptionService()
    encrypted_code = encryption_service.encrypt(code)
    assert encrypted_code != "random_code_123"
    assert len(encrypted_code) > 15

    decrypted_code = encryption_service.decrypt(encrypted_code)
    assert decrypted_code == code

