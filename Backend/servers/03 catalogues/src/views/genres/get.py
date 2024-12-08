from fastapi import Request, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel


class GenreRequest(BaseModel):
    genre: str


async def get_genres(genre: GenreRequest):
    # Get all genres that match a name
    # return them
    pass
