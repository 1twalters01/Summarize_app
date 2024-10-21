from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import DetailedBook
import jwt

async def post_force_book_creation_manually(request: Request, book: DetailedBook):
    # Admin add new Book manually [POST]

    # Check if user is admin
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

    # Scrape information
    # Return data about book to user
    pass

async def post_force_book_creation_manually_confirmation(request: Request):
    # Confirm that Book information is correct

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

    # if incorrect then remove and return deletion cancelled
    # Add book to database
    # Add action and user_uuid to log
    # return success
    pass

