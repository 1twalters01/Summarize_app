from fastapi import Request, status, retrieve_status
from fastapi.responses import JSONResponse
from pydantic import BaseModel

def post_request_author_modification(request: Request):
    # Request Author information to be modified [POST]

    # Get user uuid
    # Check that author id is valid
    # save key: uuid, value: author id
    # return success
    pass

def post_request_author_modofication_confirmation(request: Request):
    # Confirm author modification

    # Get user uuid
    # Retrieve author info from redis
    # Add user and author modification request to request database for admin to approve
    # return success
    pass

def post_force_author_modification(request: Request):
    # Admin modify Author information [POST]

    # Get user uuid
    # if rejected then delete from redis and return modification cancelled
    # update author in database
    # Add action and user_uuid to log
    # return success
    pass
