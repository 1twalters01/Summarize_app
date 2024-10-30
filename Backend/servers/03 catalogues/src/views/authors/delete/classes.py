from pydantic import BaseModel

class Author(BaseModel):
    id: str # uuid?
    reason: str
