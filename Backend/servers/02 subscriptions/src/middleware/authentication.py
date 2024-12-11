from datetime import datetime
from dotenv import load_dotenv
from fastapi import Request, HTTPException
import jwt
import os

load_dotenv()

SECRET_KEY = str(os.getenv("JWT_SECRET"))

def is_authenticated(request: Request):
    bearer: str | None = request.headers.get("bearer_token")
    if not bearer:
        raise HTTPException(status_code=400, detail="No bearer token")

    try:
        if bearer.startswith("Bearer "):
            encoded_jwt = bearer[7:].encode("utf-8")
            decoded_jwt = jwt.decode(encoded_jwt, SECRET_KEY, algorithms=["HS256"])
            user_uuid = decoded_jwt["sub"]
        else:
            raise HTTPException(status_code=401, detail="Invalid token")
    except jwt.ExpiredSignatureError:
        raise HTTPException(status_code=401, detail="Token has expired")
    except jwt.InvalidTokenError:
        raise HTTPException(status_code=401, detail="Invalid token")

    request.state.user_uuid = user_uuid

