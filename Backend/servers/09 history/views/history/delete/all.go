// delete everything from history
package views

import (
	"net/http"
	"github.com/gin-gonic/gin"
)

func DeleteAllHistory(c *gin.Context) {
	// Get user uuid
	// Delete all history items where user_uuid = user uuid (will need a join)
	// return
}
