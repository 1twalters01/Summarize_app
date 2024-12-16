// get author information
package views

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func ExampleAuthorInformation(c *gin.Context) {
	var author_name services.AuthorNameInput
	err := c.ShouldBindJSON(&author_name)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"data": "Invalid input"})
		return
	}

	author_information := services.GetAuthorInformation(author_name)
	c.JSON(http.StatusOk, gen.H{"data": "OK"})
}
