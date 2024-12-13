from fastapi import Request, HTTPException


def is_authenticated(request: Request):
    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        raise HTTPException(status_code=400, detail="No bearer token")

    try:
        encoded_jwt = bearer[:7]
        decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
        user_uuid = decoded_jwt["sub"]
    except jwt.ExpiredSignatureError:
        raise HTTPException(status_code=401, detail="Token has expired")
    except jwt.InvalidTokenError:
        raise HTTPException(status_code=401, detail="Invalid token")

    request.state.user_uuid = user_uuid