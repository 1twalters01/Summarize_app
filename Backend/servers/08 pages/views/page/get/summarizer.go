// get summariser information
package views

import (
	"net/http"
	"github.com/gin-gonic/gin"
)

func GetSummarizerInformation(c *gin.Context) {
	var summarizer_name services.SummarizerNameInput
	err := c.ShouldBindJSON(&summarizer_name)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"data": "Invalid input"})
		return
	}

	summarizer_information := services.GetSummarizerInformation(summarizer_name)
	c.JSON(http.StatusOk, gen.H{"data": "OK"})
}
