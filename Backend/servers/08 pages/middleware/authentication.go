package middleware

import (
	"net/http"
	"os"
	"strings"

	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
)

func AuthenticationMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		secretKey := os.Getenv("JWT_SECRET")
		if secretKey == "" {
			c.JSON(
                http.StatusInternalServerError,
                gin.H{"error": "Internal server error"},
            )
			c.Abort()
			return
		}

		authHeader := c.GetHeader("Authorization")
		if authHeader == "" {
			c.JSON(
                http.StatusUnauthorized,
                gin.H{"error": "Authorization header is missing"},
            )
			c.Abort()
			return
		}

		tokenString := strings.TrimPrefix(authHeader, "Bearer ")
		if tokenString == authHeader {
			c.JSON(
                http.StatusUnauthorized,
                gin.H{
                    "error": "Authorization token must be prefixed with 'Bearer '",
                },
            )
			c.Abort()
			return
		}

		// Parse and validate the token
		token, err := jwt.Parse(
            tokenString, func(token *jwt.Token) (interface{}, error) {
			    // Validate the algorithm
			    if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
				    return nil, jwt.ErrSignatureInvalid
			    }
			    return []byte(secretKey), nil
		    },
        )

		if err != nil || !token.Valid {
			c.JSON(
                http.StatusUnauthorized,
                gin.H{"error": "Invalid or expired token"},
            )
			c.Abort()
			return
		}

		// Set claims to context (if needed)
		if claims, ok := token.Claims.(jwt.MapClaims); ok {
			c.Set("userID", claims["user_id"])
		} else {
			c.JSON(
                http.StatusUnauthorized,
                gin.H{"error": "Invalid token claims"},
            )
			c.Abort()
			return
		}

		c.Next()
    }
}
