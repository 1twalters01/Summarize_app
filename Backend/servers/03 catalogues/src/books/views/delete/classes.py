from fastapi import Request, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel
import jwt

class Book(BaseModel):
    id: str
    reason: str

async def post_request_book_deletion(request: Request, book: Book):
    # Request Book information to be deleted [POST]

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

    # Check that book id is valid
    # save key: user_uuid, value: book id
    # return success
    pass

async def post_request_book_deletion_confirmation(request: Request):
    # Confirm book deletion

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

    # Retrieve book info from redis
    # Add user and book deletion request to request database for admin to approve
    # return success
    pass

async def post_force_book_deletion(request: Request, book: Book):
    # Admin delete Book information [POST]

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
    # save key: uuid, value: book id
    # return success
    pass

async def post_force_book_deletion_confirmation(request: Request):
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

    # if rejected then delete from redis and return deletion cancelled
    # remove book from database
    # Add action and user_uuid to log
    # return success
    pass
