// delete range of datetimes from history
package views

import (
	"net/http"
	"github.com/gin-gonic/gin"
)

func DeleteFromRange(c *gin.Context) {
	// Get user uuid
	// get start time and finish time

	// if neither start or finish
		// Delete all history items where user_uuid = user uuid (will need a join)
	// if start but no finish
		// delete history items where user_uuid = user uuid and date added is after start
	// if no start but finish
		// Delete history items where user_uuid = user uuid and date added is before finish
	// if start and finish
		// Delete history items where user_uuid = user uuid and date added is between start and finish

	// return
}
