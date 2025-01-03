from fastapi import Request, status
from fastapi.responses import JSONResponse
from sqlalchemy import text
from dotenv import load_dotenv
from src.queries.subscriptions.get import get_subscription_status
from src.services.encryption_service import EncryptionService
from src.utils.database_connections import get_pg_db
from src.utils.validations import validate_stripe_customer_id

load_dotenv()


async def create_stripe_customer(request: Request, stripe_customer_id: str):
    user_uuid = request.state.user_uuid

    # get user_uuid
    is_subscribed: bool | None = get_subscription_status(user_uuid)
    match is_subscribed:
        case None:
            response = {"error", "Invalid token"}
            return JSONResponse(
                content=response, status_code=status.HTTP_400_BAD_REQUEST
            )
        case True:
            response = {"error": "Customer already exists"}
            return JSONResponse(content=response, status_code=status.HTTP_409_CONFLICT)

    # Validate stripe_customer_ID
    if validate_stripe_customer_id(stripe_customer_id) == False:
        response = {"error": "Invalid stripe customer ID"}
        return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

    # set new customer
    try:
        encryption_service = EncryptionService()
        encrypted_customer_id = encryption_service.encrypt(stripe_customer_id)
        db = get_pg_db()
        if db == None:
            response = {"error": "Server error"}
            return JSONResponse(
                content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
            )
        # SET subscription_id=NULL, customer_id=encrypt(customer_id), m.method="Stripe" INTO subscriptions where user_uuid=user_uuid (joins, fix this)
        query = 'SET s.customer_id=:encrypted_customer_id, s.subscription_id=NULL, m.method="Stripe" INTO subscriptions WHERE user_uuid=:user_uuid'
        db.execute(
            text(query),
            {"encrypted_customer_id": encrypted_customer_id, "user_uuid": user_uuid},
        ).fetchone()
        response = {"success": True}
        return JSONResponse(content=response, status_code=status.HTTP_200_OK)
    except:
        response = {"success": False}
        return JSONResponse(
            content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
        )
