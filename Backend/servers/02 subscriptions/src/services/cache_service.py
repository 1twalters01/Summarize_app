from redis import Redis

class CacheService():
    client: Redis

    def __init__(self, client: Redis):
        self.client = client

    def store_value_for_key(value: str, key: str, ex: int|None):
        if ex == None:
            r.set(key, value)
        elif type(ex) == int:
            r.set(key, ex)
        else:
            return TypeError

    def get_value_from_key(key: str):
        value = r.get(key)
        return value

    def delete_key(key: str):
        r.delete(key)
