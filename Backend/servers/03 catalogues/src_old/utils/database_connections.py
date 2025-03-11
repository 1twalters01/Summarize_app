import os
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, Session
from redis import Redis
from dotenv import load_dotenv

load_dotenv()


def get_pg_db() -> Session | None:
    pg_url = os.getenv("PG_URL")
    if pg_url == None:
        return None
    try:
        engine = create_engine(pg_url)
        SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
        db = SessionLocal()
        return db
    except:
        return None


async def create_redis_connection() -> Redis | None:
    redis_url = os.getenv("REDIS_URL")
    if redis_url == None:
        return None
    return Redis.from_url(redis_url)
