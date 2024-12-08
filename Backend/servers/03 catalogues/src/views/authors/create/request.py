from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Author
import jwt


async def post_request_author_creation(request: Request, author: Author):
    # Request new Author to be added [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Scrape information Return data about author to user
    # Save key: user_uuid, value: author info (or similar) to redis
    # return success
    pass


async def post_request_author_creation_confirmation(request: Request):
    # Confirm that author scraped is correct [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # if incorrect author then remove from database and return error?
    # Retrieve author info from redis
    # Add user and author creation request to request database to be approved by admin
    # return success
    pass
