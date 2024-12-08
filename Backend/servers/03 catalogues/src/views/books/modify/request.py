from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Book
import jwt


async def post_request_book_modification(request: Request, book: Book):
    # Request Book information to be modified [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Check that book id is valid
    # save key: uuid, valid: book id
    # return success
    pass


async def post_request_book_modification_confirmation(request: Request):
    # Confirm book modification

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Retrieve book info from redis
    # Add user and book modification request to request database for admin to approve
    # return success
    pass
