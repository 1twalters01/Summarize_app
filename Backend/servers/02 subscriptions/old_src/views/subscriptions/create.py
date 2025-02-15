from fastapi import Request, status

from enum import Enum
class CreateMethod(Enum):
    Stripe = 1
    Paypal = 2

# Why did I get the customer id? Just fetch their user id?
async def create_subscription_view(request: Request, method: CreateMethod):
    user_uuid = request.state.user_uuid

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

    if method == CreateMethod.Stripe:
        if validate_stripe_customer_id(customer_id) == False:
            response = {"error": "Invalid stripe customer ID"}
            return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

        # set new customer
        try:
            encryption_service = EncryptionService()
            encrypted_customer_id = encryption_service.encrypt(customer_id)
            db = get_pg_db()
            if db == None:
                response = {"error": "Server error"}
                return JSONResponse(
                    content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
                )
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

    if method == CreateMethod.Paypal:
        if validate_paypal_customer_id(customer_id) == False:
            response = {"error": "Invalid stripe customer ID"}
            return JSONResponse(content=response, status_code=status.HTTP_400_BAD_REQUEST)

        # set new customer
        try:
            encryption_service = EncryptionService()
            encrypted_customer_id = encryption_service.encrypt(customer_id)
            db = get_pg_db()
            if db == None:
                response = {"error": "Server error"}
                return JSONResponse(
                    content=response, status_code=status.HTTP_500_INTERNAL_SERVER_ERROR
                )
            query = 'SET s.customer_id=:encrypted_customer_id, s.subscription_id=NULL, m.method="Paypal" INTO subscriptions WHERE user_uuid=:user_uuid'
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