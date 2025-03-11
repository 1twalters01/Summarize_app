from pydantic import BaseModel


class Genre(BaseModel):
    genre: str
    definition: str  # A definition to ensure it is the correct one


class DetailedGenre(BaseModel):
    genre: str
    definition: str
    subgenres: list[str]
    supergenres: list[str]
