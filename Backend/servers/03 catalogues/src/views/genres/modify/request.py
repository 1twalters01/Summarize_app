from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Genre
import jwt

async def post_request_genre_modification(request: Request, genre: Genre):
    # Request Genre information to be modification [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(
            content=response, status_code=status.HTTP_400_BAD_REQUEST
        )

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Check that genre id is valid
    # save key: uuid, value: genre id
    # return success
    pass

async def post_request_genre_modification_confirmation(request: Request):
    # Confirm genre modification

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(
            content=response, status_code=status.HTTP_400_BAD_REQUEST
        )

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Retrieve genre info from redis
    # Add user and genre modification request to request database for admin to approve
    # return success
    pass

