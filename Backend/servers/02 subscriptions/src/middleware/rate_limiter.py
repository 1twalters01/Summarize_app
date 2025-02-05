from datetime import datetime
from dotenv import load_dotenv
from fastapi import Request, HTTPException, status
import os

load_dotenv()

SECRET_KEY = str(os.getenv("JWT_SECRET"))

def rate_limiter(request: Request):
    ip = request.client.host
    redis_key = f"{client_ip}"

    current_count = redis_client.get(redis_key)
    if current_count is None:
        redis_client.set(redis_key, 1, ex=expiry_in_seconds)
    else:
        current_count = int(current_count)
        if current_count >= limit:
            raise HTTPException(status_code=429, detail="Too many requests")
        redis_client.incr(redis_key)
