from fastapi import status
from fastapi.responses import JSONResponse

async def get_any_auth():
    response = {"message": "Ping any authorisation level from server"}
    return JSONResponse(content=response, status_code=status.HTTP_200_OK)

