from cryptography.fernet import Fernet

# Need to properly get key
key = "key"

def encrypt(token):
    fernet = Fernet(key)
    encoded_token = token.encode('ascii')
    encrypted_token = fernet \
            .encrypt(token) \
            .decode('ascii')
    formatted_token = encrypted_token.lstrip("b'") \
            .rstrip("'") \
            .replace("=", "%")
    return formatted_token

def decrypt(token):
    fernet = Fernet(key)
    formatted_token = token.replace("%", "=") \
            .encode('ascii')
    decrypted_token = fernet.decrypt(formatted_token) \
            .decode('ascii')
    return decrypted_token
