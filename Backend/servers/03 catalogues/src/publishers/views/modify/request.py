from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Publisher
import jwt


async def post_request_publisher_modification(request: Request, publisher: Publisher):
    # Request Publisher information to be modified [POST]

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

    # Check that publisher id is valid
    # save key: uuid&modification_request, value: publisher id
    # return success
    pass

async def post_request_publisher_modification_confirmation(request: Request):
    # Confirm author modification
    
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
    # Retrieve publisher info from redis
    # Add user and publisher modification request to request database for admin to approve
    # return success
    pass

