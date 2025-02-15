from datetime import datetime
from dotenv import load_dotenv
from fastapi import Request, HTTPException, status
import os

async def verify_captcha_token(request: Request):
    captcha_header = request.headers.get("Captcha")
    if not captcha_header:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Captcha header is missing",
        )

    try:
        decoded_token = jwt.decode(captcha_header, SECRET_KEY, algorithms=["HS256"])
        expiration_time = decoded_token.get("exp")
        request_ip = request.client.host
        token_ip = decoded_token.get("ip")

        if expiration_time < datetime.utcnow().timestamp():
            raise HTTPException(
                status_code=status.HTTP_401_UNAUTHORIZED,
                detail="Invalid or expired token"
            )

        if request_ip != token_ip:
            raise HTTPException(
                status_code=status.HTTP_401_UNAUTHORIZED,
                detail="Invalid IP address",
            )

        return decoded_token

    except jwt.ExpiredSignatureError:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Token has expired",
        )
    except jwt.InvalidTokenError:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid token claims",
        )
