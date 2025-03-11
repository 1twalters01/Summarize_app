from pydantic import BaseModel


class Publisher(BaseModel):
    name: str
    owner: str


class DetailedPublisher(BaseModel):
    name: str
    owner: str
