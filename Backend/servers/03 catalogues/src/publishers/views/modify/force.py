from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Publisher
import jwt

async def post_force_publisher_modification(request: Request, publisher: Publisher):
    # Admin modify Publisher information [POST]

    # Get user uuid
    bearer: str|None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(
            content=response,
            status_code=status.HTTP_400_BAD_REQUEST
        )
    
    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Check if user is admin
    # save key: uuid&request, value: publisher id
    # return success
    pass

async def post_force_publisher_modification_confirmation(request: Request):
    # Confirmation for admin delete post

    # Get user uuid
    bearer: str|None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(
            content=response,
            status_code=status.HTTP_400_BAD_REQUEST
        )
    
    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # if rejected then delete from redis and return modification cancelled
    # remove publisher from database
    # Add action and user_uuid to log
    # return success
    pass
