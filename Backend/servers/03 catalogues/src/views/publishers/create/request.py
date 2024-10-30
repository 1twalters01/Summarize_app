from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Publisher
import jwt

async def post_request_publisher_creation(request: Request, publisher: Publisher):
    # Request new Publisher to be added [POST]
    
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

    # Scrape information and return data about publisher to user
    # Save key: user_uuid, value: publisher info to redis (or similar)
    # return success
    pass
    
async def post_request_publisher_creation_confirmation(request: Request):
    # Admin add new Publisher [POST]

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

    # if incorrect publisher then remove from database and return error?
    # Retrieve publisher info from redis
    # Add user and publisher creation request to request database to be approved by admin
    # return success
    pass

