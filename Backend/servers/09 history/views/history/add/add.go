// Add to history
package views

import (
	"net/http"
	"github.com/gin-gonic/gin"
)

func AddHistoryItem(c *gin.Context) {
	// Get user uuid
	user_uuid, exists := c.Get("userID")
	if !exists {
		c.JSON(http.StatusUnauthorized, gin.H{"error": "User ID not found in context"})
		return
	}
    
    // Get type and uuid of history item
	// Add to db
	// Add to cache and remove last thing of the same type in the cache
	// return
}
