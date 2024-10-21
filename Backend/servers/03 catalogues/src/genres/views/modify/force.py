from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Genre
import jwt

async def post_force_genre_modification(request: Request, genre: Genre):
    # Admin modify Genre information [POST]

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

    # Check if user is admin
    # save key: uuid&genresModifyForce, value: genre id
    # return success
    pass

async def post_force_genre_modification_confirmation(request: Request):
    # Confirmation for admin modification post

    # if rejected then delete from redis and return modification cancelled
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

    # update genre in database
    # Remove genre from redis
    # Add action and user_uuid to log
    # return success
    pass
