from fastapi import Request, status
from fastapi.responses import JSONResponse
import jwt

# Use rust for type safety for this bit?
class ChangeRequest(BaseModel):
    id: str # id of change request
    type: DatatypeEnum # Author, Book, Genre, Publisher
    fields: fieldsEnum # Based on DatatypeEnum

def modify_single_change_request(request: Request, change_request: ChangeRequest): # uuid should be uuid instead of str?
    # Admin modify specific change request [POST]

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
    
    # Check if user is admin
    # Ensure fields corresponds to type
    # Update the data in the requests database
    # return result
    pass

