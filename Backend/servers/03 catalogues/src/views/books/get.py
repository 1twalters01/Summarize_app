from fastapi import Request, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel

class BookRequest(BaseModel):
    title: str
    author_name: str

async def get_books(book_name: BookRequest):
    # Get all book ids that match a name
    # return them
    pass
