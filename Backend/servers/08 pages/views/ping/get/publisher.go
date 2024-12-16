// get publisher information
package views

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func ExamplePublisherInformation(c *gin.Context) {
	var publisher_name services.PublisherNameInput
	err := c.ShouldBindJSON(&publisher_name)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"data": "Invalid input"})
		return
	}

	publisherr_information := services.GetPublisherInformation(publisher_name)
	c.JSON(http.StatusOk, gen.H{"data": "OK"})
}
