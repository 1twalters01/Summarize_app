from fastapi import status
from fastapi.responses import JSONResponse
from pydantic import BaseModel


async def get_any_auth():
    response = {"message": "Ping any authorisation level from server"}
    return JSONResponse(content=response, status_code=status.HTTP_200_OK)


class Data(BaseModel):
    message: str


async def post_any_auth(data: Data):
    message_1 = "Ping any authorisation level from server"
    message_2 = "Request data: " + data.message
    response = {message_1: message_1, message_2: message_2}
    return JSONResponse(content=response, status_code=status.HTTP_200_OK)
