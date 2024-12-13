def test_no_uuid():
    mock_request = MagicMock(spec=Request)
    mock_request.headers = {}

    with pytest.raises(HTTPException) as exception_info:
        is_admin(mock_request)
    assert exc_info.value.status_code == 500
    assert exc_info.value.detail == "User not authenticated properly"

def test_server_error():
    mock_request = MagicMock(spec=Request)
    mock_request.headers = {}
    mock_request.state.user_uuid = str(os.getenv("INVALID_TEST_UUID"))

    with pytest.raises(HTTPException) as exception_info:
        is_admin(mock_request)
    assert exc_info.value.status_code == 403
    assert exc_info.value.detail == "Server error"

def test_not_admin():
    mock_request = MagicMock(spec=Request)
    mock_request.headers = {}
    mock_request.state.user_uuid = str(os.getenv("NOT_ADMIN_TEST_UUID"))

    with pytest.raises(HTTPException) as exception_info:
        is_admin(mock_request)
    assert exc_info.value.status_code == 403
    assert exc_info.value.detail == "Not an admin"


def test_is_admin():
    mock_request = MagicMock(spec=Request)
    mock_request.headers = {}
    mock_request.state.user_uuid = str(os.getenv("ADMIN_TEST_UUID"))

    is_admin(mock_request)