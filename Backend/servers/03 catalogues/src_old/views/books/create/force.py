import os
from fastapi import Request, status
from fastapi.responses import JSONResponse
from sqlalchemy import create_engine

from src.queries.user.get import get_admin_status
from .classes import Book
import jwt


async def post_force_book_creation(request: Request, book: Book):
    # Admin add new Book [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Check if user is admin
    pg_url = os.getenv("PG_URL")
    if pg_url == None:
        response = {"error", "Internal Server Error"}
        return JSONResponse(
            content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
        )

    engine = create_engine(pg_url)
    if get_admin_status(engine, user_uuid) == False:
        response = {"error", "not admin"}
        return JSONResponse(content=response, status_code=status.HTTP)

    # Scrape information
    # Save token: book to redis
    # Return data about book to user
    pass


async def post_force_book_creation_confirmation(request: Request):
    # Confirm that book scraped is correct [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # if incorrect then remove
    # get book from redis
    # Add author to database
    # return success
    pass
