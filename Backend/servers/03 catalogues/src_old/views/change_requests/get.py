from fastapi import Request, status
from fastapi.responses import JSONResponse
import jwt


def get_change_requests(request: Request):
    # Admin view change requests [GET]

    # Get user uuid
    bearer: str | None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    encoded_jwt = bearer[7:]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Check if user is admin
    # Get all requests from request database and return data includes (id, title, type, etc.)
    pass


def get_single_change_request(
    request: Request, id: str
):  # uuid should be uuid instead of str?
    # Admin view specific change request [GET]

    # Get user uuid
    # Check if user is admin
    # Select * from request database where id = (something)
    # return result
    pass
