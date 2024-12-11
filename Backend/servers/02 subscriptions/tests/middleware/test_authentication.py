from src.middleware.authentication import is_authenticated
from unittest.mock import MagicMock
from fastapi import Request, HTTPException
import jwt
import pytest
from dotenv import load_dotenv
import os
import uuid

load_dotenv()

SECRET_KEY = str(os.getenv("JWT_SECRET"))
ALGORITHM = "HS256"

def test_no_bearer_token():
    mock_request = MagicMock(spec=Request)
    mock_request.headers = {}

    with pytest.raises(HTTPException) as exception_info:
        is_authenticated(mock_request)
    assert exception_info.value.status_code == 400
    assert exception_info.value.detail == "No bearer token"
    
def test_expired_token():
    expired_token = str(os.getenv("EXPIRED_TOKEN"))
    mock_request = MagicMock(spec=Request)
    mock_request.headers = {"bearer_token": f"Bearer {expired_token}"}

    with pytest.raises(HTTPException) as exception_info:
        is_authenticated(mock_request)
    assert exception_info.value.status_code == 401
    assert exception_info.value.detail == "Token has expired"

def test_invalid_token():
    invalid_token = "invalid.token"
    mock_request = MagicMock(spec=Request)
    mock_request.headers = {"bearer_token": f"Bearer {invalid_token}"}

    with pytest.raises(HTTPException) as exception_info:
        is_authenticated(mock_request)
    assert exception_info.value.status_code == 401
    assert exception_info.value.detail == "Invalid token"

def test_valid_bearer_token():
    test_uuid = uuid.uuid4()
    valid_token = jwt.encode({"sub": str(test_uuid)}, SECRET_KEY, algorithm=ALGORITHM)
    mock_request = MagicMock(spec=Request)
    mock_request.headers = {"bearer_token": f"Bearer {valid_token}"}

    is_authenticated(mock_request)
    assert uuid.UUID(mock_request.state.user_uuid) == test_uuid 
