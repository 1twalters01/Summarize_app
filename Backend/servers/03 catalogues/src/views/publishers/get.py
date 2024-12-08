from fastapi import Request, status, retrieve_status
from fastapi.responses import JSONResponse
from pydantic import BaseModel


class PublisherRequest(BaseModel):
    title: str
    author_name: str


def get_publishers(publisher_name: PublisherRequest):
    # Get all publisher ids that match a name
    # return them
    pass
