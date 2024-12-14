// authentication test

package authentication_middleware_test

import (
    "net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

    "pages/middleware"
	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
	"github.com/stretchr/testify/assert"
)

func generateTestJWT(secret string, userID string) string {
    claims := jwt.MapClaims{
        "user_id": userID,
        "exp": time.Now().Add(time.Hour * 24).Unix(),
    }
    token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
    tokenString, _ := token.SignedString([]byte(secret))
    return tokenString
}

func TestAuthMiddleWare_MissingHeader(t *testing.T) {
    gin.SetMode(gin.TestMode)
    
    w := httptest.NewRecorder()
    c, _ := gin.CreateTestContext(w)
    c.Request = httptest.NewRequest("GET", "/protected", nil)

    middleware.AuthenticationMiddleware()(c)

    assert.Equal(t, http.StatusUnauthorized, w.Code)
    assert.Contains(t, w.Body.String(), "Authorization header is missing")
}

func TestAuthMiddleware_InvalidToken(t *testing.T) {
    gin.SetMode(gin.TestMode)
    invalidToken := "invalid_jwt_token"

    w := httptest.NewRecorder()
    c, _ := gin.CreateTestContext(w)
    c.Request = httptest.NewRequest("GET", "/protected", nil)
    c.Request.Header.Set("Authorization", "Bearer "+invalidToken)

    middleware.AuthenticationMiddleware()(c)

    assert.Equal(t, http.StatusUnauthorized, w.Code)
    assert.Contains(t, w.Body.String(), "Invalid or expired token")
}

func TestAuthMiddleware_ValidToken(t *testing.T) {
    gin.SetMode(gin.TestMode)
	secretKey := os.Getenv("JWT_SECRET")
    validToken := generateTestJWT(secretKey, "test_user")

    w := httptest.NewRecorder()
    c, _ := gin.CreateTestContext(w)
    c.Request = httptest.NewRequest("GET", "/protected", nil)
    c.Request.Header.Set("Authorization", "Bearer "+validToken)

    middleware.AuthenticationMiddleware()(c)

    assert.Equal(t, http.StatusOK, w.Code)
    assert.Nil(t, c.Errors)
    userID, exists := c.Get("userID")
    assert.True(t, exists)
    assert.Equal(t, "test_user", userID)
}
