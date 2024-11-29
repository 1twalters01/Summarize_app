from fastapi import Request, HTTPException
import jwt

def is_authenticated(request: Request):
    # Get user uuid
    bearer: str|None = request.headers.get("bearer_token")
    if bearer == None:
        response = {"error", "no token"}
        raise HTTPException(status_code=400, detail="No bearer token")
    
    encoded_jwt = bearer[:7]
    decoded_jwt = jwt.decode(encoded_jwt, "secret", algorithms=["HS256"])
    user_uuid = decoded_jwt["sub"]

    # Check if user is admin, move to middleware later
    conn = create_pg_connection()
    if type(conn) is str:
        raise HTTPException(status_code=500, detail="db connection error")
    elif type(conn) is Engine:
        if get_admin_status(conn, user_uuid) == False:
            raise HTTPException(status_code=400, detail="Not an admin")