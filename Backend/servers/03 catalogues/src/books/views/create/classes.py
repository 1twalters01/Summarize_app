from pydantic import BaseModel

class Book(BaseModel):
    Title: str
    last_name: str
    author_first_name: str # Author name to ensure they are the correct one
    author_last_name: str

class DetailedBook(BaseModel):
    title: str
    author: str
    genres: list[str]
