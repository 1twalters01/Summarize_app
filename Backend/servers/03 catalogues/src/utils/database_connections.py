import os
from sqlalchemy import create_engine, Engine
from redis import Redis

def create_pg_connection() -> Engine|str:
    pg_url = os.getenv("PG_URL")
    if pg_url == None:
        return "Internal Server Error"

    return create_engine(pg_url)

async def create_redis_connection() -> Redis|str:
    redis_url = os.getenv("REDIS_URL")
    if redis_url == None:
        return "Internal Server Error"
    return Redis.from_url(redis_url)

