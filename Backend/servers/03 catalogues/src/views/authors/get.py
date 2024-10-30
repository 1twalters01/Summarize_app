from fastapi import Request, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel

class AuthorRequest(BaseModel):
    first_name: str
    last_name: str
    middle_name: str|None

async def get_authors(author_name: AuthorRequest):
    # Get all author ids that match a name
    # return them
    pass
