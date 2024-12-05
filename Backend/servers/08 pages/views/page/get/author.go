// get author information
package views

import (
	"net/http"
	"github.com/gin-gonic/gin"
)

func GetAuthorInformation(c *gin.Context) {
	var author_name services.AuthorNameInput
	err := c.ShouldBindJSON(&author_name)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"data": "Invalid input"})
		return
	}

	author_information := services.GetAuthorInformation(author_name)
	c.JSON(http.StatusOk, gen.H{"data": "OK"})
}
