from fastapi import Request, status, retrieve_status
from fastapi.responses import JSONResponse
from pydantic import BaseModel

class Author(BaseModel):
    id: str # uuid?
    reason: str

def post_request_author_deletion(request: Request, author: Author):
    # Request Author information to be deleted [POST]

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

    # Check that author id is valid
    # save key: uuid, value: author id
    # return success
    pass

def post_request_author_deletion_confirmation(request: Request):
    # Confirm author deletion

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

    # Retrieve author info from redis
    # Add user and author deletion request to request database for admin to approve
    # return success
    pass

def post_force_author_deletion(request: Request, author: Author):
    # Admin delete Author information [POST]

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
    # save key: uuid, value: author id
    # return success
    pass

def post_foce_author_deletion_confirmation(request: Request):
    # Confirmation for admin delete post

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

    # if rejected then delete from redis and return deletion cancelled
    # remove author from database
    # Add action and user_uuid to log
    # return success
    pass