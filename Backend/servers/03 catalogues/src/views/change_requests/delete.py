from fastapi import Request, status
from fastapi.responses import JSONResponse
import jwt

def post_change_request_deletion(request: Request, id: list[str]):
    # Admin delete specific change request [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(
            content=response, status_code=status.HTTP_400_BAD_REQUEST
        )

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]
    
    # save key: user_uuid, value: id
    # Check if user is admin
    pass
    
def post_change_request_deletion_confirmation(request: Request):
    # Admin delete specific change request confirmation [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(
            content=response, status_code=status.HTTP_400_BAD_REQUEST
        )

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Get ids from redis using user_uuid 
    # Delete all change requests in change request database where id is equal to an id
    pass
