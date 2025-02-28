from redis import Redis

class CacheService():
    client: Redis

    def __init__(self, client: Redis):
        self.client = client

    def store_value_for_key(self, value: str, key: str, expiration_in_seconds: int|None):
        if expiration_in_seconds == None:
            self.client.set(key, value)
        elif type(expiration_in_seconds) == int:
            self.client.set(key, expiration_in_seconds)
        else:
            return TypeError

    def get_value_from_key(self, key: str):
        value = self.client.get(key)
        return value

    def delete_key(self, key: str):
        self.client.delete(key)

    def store_discount_code_for_user_uuid(self, discount_code: str, user_uuid: str, expiration_in_seconds: int|None):
        key = f"discount for {user_uuid}"
        value = discount_code
        self.store_value_for_key(value, key, expiration_in_seconds)