from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import DetailedGenre
import jwt

async def post_force_genre_creation_manually(request: Request, genre: DetailedGenre):
    # Admin add new Genre manually [POST]

    # Get user uuid
    # Check if user is admin
    # Scrape information
    # Return data about genre to user
    pass

async def post_force_genre_creation_manually_confirmation(request: Request):
    # Confirm that genre information is correct

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

    # If incorrect then remove and return creation cancelled
    # Add genre to database
    # Add action and user_uuid to log
    # return success
    pass
