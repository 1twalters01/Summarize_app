from fastapi import Request, status, retrieve_status
from fastapi.responses import JSONResponse
from pydantic import BaseModel

class Book(BaseModel):
    Title: str
    last_name: str
    author_first_name: str # Author name to ensure they are the correct one
    author_last_name: str

class DetailedBook(BaseModel):
    title: str
    author: str
    genres: list[str]

def post_request_book_creation(request: Request):
    # Request new Book to be added [POST]

    # Get user uuid
    bearer: str|None = Request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(
            content=response,
            status_code=status.HTTP_400_BAD_REQUEST
        )
    
    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Scrape information Return data about book to user
    # Save key: user_uuid, value: book info (or similar) to redis
    # return success
    pass

def post_request_book_creation_confirmation(request: Request):
    # Request new Author to be added [POST]

    # Get user uuid
    bearer: str|None = Request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(
            content=response,
            status_code=status.HTTP_400_BAD_REQUEST
        )
    
    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # if incorrect book then remove from database and return error?
    # Retrieve book info from redis
    # Add user and book creation request to request database to be approved by admin
    # return success
    pass

def post_force_book_creation(request: Request, book: Book):
    # Admin add new Book [POST]

    # Get user uuid
    bearer: str|None = Request.headers.get("bearer_token")
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
    # Return data about book to user
    pass

def post_force_book_creation_confirmation(request: Request):
    # Confirm that book scraped is correct [POST]

    # Get user uuid
    bearer: str|None = Request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(
            content=response,
            status_code=status.HTTP_400_BAD_REQUEST
        )
    
    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # if incorrect then remove
    # Add author to database
    # return success
    pass

def post_force_book_creation_manually(request: Request, book: DetailedBook):
    # Admin add new Book manually [POST]

    # Check if user is admin
    # Scrape information
    # Return data about book to user
    pass

def post_force_book_creation_manually_confirmation(request: Request):
    # Confirm that Book information is correct

    # Get user uuid
    bearer: str|None = Request.headers.get("bearer_token")
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
