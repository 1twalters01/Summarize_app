from pydantic import BaseModel


class Book(BaseModel):
    id: str
    reason: str
