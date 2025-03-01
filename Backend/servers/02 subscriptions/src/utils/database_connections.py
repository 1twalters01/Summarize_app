import os
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, Session
from dotenv import load_dotenv
from redis import Redis

load_dotenv()


def get_pg_db() -> Session | None:
    DATABASE_URL = os.getenv("PG_URL")
    if DATABASE_URL == None:
        return None
    try:
        engine = create_engine(DATABASE_URL)
        SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
        db = SessionLocal()
        return db
    except:
        return None

def get_redis_client_connection() -> Redis:
    url = os.getenv("REDIS_URL")
    
    try:
        client = Redis.from_url(url)
        return client
    except redis.RedisError as e:
        raise RuntimeError(f"Failed to connect to Redis: {e}")
