from fastapi import Request, status
from fastapi.responses import JSONResponse
from .classes import ChangeRequest
import jwt


def post_accept_change_request(request: Request, change_request: ChangeRequest):
    # Admin accept change request (with posible modifications made) [POST]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Check if user is admin
    # Ensure fields corresponds to type
    # match type:
    # if author then update author fields where id = (id)
    # if book then update book fields where id = (id)
    # if genre then update genre fields where id = (id)
    # if publisher then update publisher fields where id = (id)
    # else return error
    # return result
    pass
