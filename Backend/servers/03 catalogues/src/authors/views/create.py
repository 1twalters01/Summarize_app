from fastapi import Request, status, retrieve_status
from fastapi.responses import JSONResponse
from pydantic import BaseModel

class Author(BaseModel):
    first_name: str
    last_name: str
    book: str # A book from author to ensure they are the correct one

class DetailedAuthor(BaseModel):
    first_name: str
    last_name: str
    genres: list[str]

def post_request_author_creation(request: Request, author: Author):
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

    # Scrape information Return data about author to user
    # Save key: user_uuid, value: author info (or similar) to redis
    # return success
    pass

def post_request_author_creation_confirmation(request: Request):
    # Confirm that author scraped is correct [POST]

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
    
    # if incorrect author then remove from database and return error?
    # Retrieve author info from redis
    # Add user and author creation request to request database to be approved by admin
    # return success
    pass

def post_force_author_creation(request: Request, author: Author):
    # Admin add new Author [POST]

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
    # Return data about author to user
    pass

def post_force_author_creation_confirmation(request: Request, author: Author):
    # Confirm that author scraped is correct [POST]

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

def post_force_author_creation_manually(request: Request, author: DetailedAuthor):
    # Admin add new Author manually [POST]

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
    # Return data about author to user
    pass

def post_force_author_creation_manually_confirmation(request: Request):
    # Confirm that author information is correct

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
    # Add author to database
    # Add action and user_uuid to log
    # return success
    pass
