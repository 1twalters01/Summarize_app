from fastapi import Request, HTTPException
import jwt
from src.queries.user.get import get_admin_status


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

    # Check if user is admin
    admin_status = get_admin_status(user_uuid)
    match admin_status:
        case False:
            raise HTTPException(status_code=403, detail="Not an admin")
        case None:
            raise HTTPException(status_code=500, detail="Server error")

    request.state.user_uuid = user_uuid
