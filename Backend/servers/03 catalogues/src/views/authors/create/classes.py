from pydantic import BaseModel


class Author(BaseModel):
    first_name: str
    last_name: str
    book: str  # A book from author to ensure they are the correct one


class DetailedAuthor(BaseModel):
    first_name: str
    last_name: str
    genres: list[str]
