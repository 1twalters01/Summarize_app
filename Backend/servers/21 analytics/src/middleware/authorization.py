from fastapi import Request, HTTPException
from src.queries.user.get import get_admin_status


def is_admin(request: Request):
    if not hasattr(request.state, "user_uuid"):
        raise HTTPException(status_code=500, detail="User not authenticated properly")

    user_uuid = request.state.user_uuid
    admin_status = get_admin_status(user_uuid)
    match admin_status:
        case False:
            raise HTTPException(status_code=403, detail="Not an admin")
        case None:
            raise HTTPException(status_code=500, detail="Server error")
