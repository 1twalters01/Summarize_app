from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Author
import jwt


async def post_request_author_deletion(request: Request, author: Author):
    # Request Author information to be deleted [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Check that author id is valid
    # save key: uuid, value: author id
    # return success
    pass


async def post_request_author_deletion_confirmation(request: Request):
    # Confirm author deletion

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Retrieve author info from redis
    # Add user and author deletion request to request database for admin to approve
    # return success
    pass
