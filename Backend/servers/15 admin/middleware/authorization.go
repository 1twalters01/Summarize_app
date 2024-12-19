// Authorization middleware
package middleware

import (
	"net/http"
	"github.com/gin-gonic/gin"
    "history/queries"
)

func AuthorizationMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
        userUUID, exists := c.Get("user_uuid")
        if !exists {
            c.JSON(
                http.StatusUnauthorized,
                gin.H{"error": "User not authenticated properly"},
            )
            return
        }

        _, ok := userUUID.(string)
        if !ok {
            c.JSON(
                http.StatusInternalServerError,
                gin.H{"error": "Invalid user ID type"},
            )
            return
        }

        admin_status, err := queries.get_admin_status(userUUID)
        if err != nil {
            c.JSON(
                http.StatusInternalServerError,
                gin.H{"error": "Failed to check admin status"},
            )
            c.Abort()
            return
        }
        if !admin_status {
            c.JSON(
                http.StatusForbidden,
                gin.H{"error": "Not an admin"},
            )
        }

        c.Next()
    }
}

