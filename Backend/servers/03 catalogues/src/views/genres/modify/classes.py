from pydantic import BaseModel

class Genre(BaseModel):
    id: str # uuid?
    reason: str
