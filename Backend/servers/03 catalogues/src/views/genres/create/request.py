from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Genre
import jwt

async def post_request_genre_creation(request:Request, genre: Genre):
    # Request new Genre to be added [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(
            content=response, status_code=status.HTTP_400_BAD_REQUEST
        )

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]
    
    # Scrape information return data about genre to user
    # Save key: user_uuid&genreCreateRequest, value: genre info to redis
    # return success
    pass

async def post_request_genre_creation_confirmation(request: Request):
    # Confirm that genre scraped is correct [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(
            content=response, status_code=status.HTTP_400_BAD_REQUEST
        )

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]
    
    # if incorrect genre then remove from redis and return error?
    # Retrieve genre info from redis
    # Add user and genre creation request to request database to be approved by admin
    # return success
    pass
