from fastapi import status
from fastapi.responses import JSONResponse
from pydantic import BaseModel

async def get_only_auth():
    response = {"message": "Ping only authorisation level from server"}
    return JSONResponse(content=response, status_code=status.HTTP_200_OK)

class Data(BaseModel):
    message: str

async def post_only_auth(data: Data):
    message_1 = "Ping only authorisation level from server"
    message_2 = "Request data: " + data.message
    response = {message_1: message_1, message_2: message_2}
    return JSONResponse(content=response, status_code=status.HTTP_200_OK)

