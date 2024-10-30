from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import DetailedPublisher
import jwt

async def post_force_publisher_creation_manually(request: Request, publisher: DetailedPublisher):
    # Admin add new publisher manually
    
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
    # Scrape information
    # Return data about publisher to user
    pass

async def post_force_publisher_creation_manually_confirmation(request: Request):
    # Confirm that publisher information is correct

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

    # if incorrect thjen remove
    # Add publisher to datbase
    # Add action and user_uuid to log
    # return success
    pass

