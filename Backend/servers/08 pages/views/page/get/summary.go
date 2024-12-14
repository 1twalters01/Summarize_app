// get summary information
package views

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func GetSummaryInformation(c *gin.Context) {
	var summary_name services.SummaryNameInput
	err := c.ShouldBindJSON(&summary_name)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"data": "Invalid input"})
		return
	}

	summary_information := services.GetSummaryInformation(summary_name)
	c.JSON(http.StatusOk, gen.H{"data": "OK"})
}
