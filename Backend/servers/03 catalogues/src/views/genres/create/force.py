from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Genre
import jwt
from sqlalchemy import Engine
from ....queries.user.get import get_admin_status
from ....utils.database_connections import create_pg_connection


async def post_force_genre_creation(request: Request, genre: Genre):
    # Admin add new Genre [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Check if user is admin
    conn = create_pg_connection()
    if type(conn) is str:
        return JSONResponse(
            content={"error": conn}, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
        )
    elif type(conn) is Engine:
        if get_admin_status(conn, user_uuid) == False:
            response = {"error", "not admin"}
            return JSONResponse(
                content=response, status_code=status.HTTP_401_UNAUTHORIZED
            )

    # Scrape information
    # Return data about genre to user
    pass


async def post_force_genre_creation_confirmation(request: Request, genre: Genre):
    # Confirm that genre is scraped correctly [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # if incorrect then remove
    # Add genre to datbase
    # Add all books containing supergenre of the genre to suspect list
    # return success
    pass
