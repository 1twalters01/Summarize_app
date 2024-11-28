from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import Author, DetailedAuthor
import jwt
from sqlalchemy import Engine
from ....queries.user.get import get_admin_status
from ....utils.database_connections import create_pg_connection

async def post_force_author_creation(request: Request, author: Author):
    # Admin add new Author [POST]

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

    # Check if user is admin, move to middleware later
    conn = create_pg_connection()
    if type(conn) is str:
        return JSONResponse(
            content={"error": conn},
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
        )
    elif type(conn) is Engine:
        if get_admin_status(conn, user_uuid) == False:
            response = {"error", "not admin"}
            return JSONResponse(
                content=response,
                status_code=status.HTTP_401_UNAUTHORIZED
            )
        
    # Scrape information
    author_information = scrape_author(author)

    # Return data about author to user
    pass

async def post_force_author_creation_confirmation(request: Request, author: DetailedAuthor|None):
    # Confirm that author scraped is correct [POST]

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

    # Check if user is admin
    conn = create_pg_connection()
    if type(conn) is str:
        return JSONResponse(
            content={"error": conn},
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
        )
    elif type(conn) is Engine:
        if get_admin_status(conn, user_uuid) == False:
            response = {"error", "not admin"}
            return JSONResponse(
                content=response,
                status_code=status.HTTP_401_UNAUTHORIZED
            )

    # Add author to database
    # return success
    pass
