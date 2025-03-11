from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Genre
import jwt


async def post_request_genre_deletion(request: Request, genre: Genre):
    # Request Genre information to be deleted [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Check that genre id is valid
    # save key: uuid, value: genre id
    # return success
    pass


async def post_request_genre_deletion_confirmation(request: Request):
    # Confirm genre deletion

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Retrieve genre info from redis
    # Add user and genre deletion request to request database for admin to approve
    # return success
    pass
