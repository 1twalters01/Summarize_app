from pydantic import BaseModel

class Publisher(BaseModel):
    id: str
    reason: str

