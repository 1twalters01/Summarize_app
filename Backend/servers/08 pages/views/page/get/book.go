// get book information
package views

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func GetBookInformation(c *gin.Context) {
	var book_name services.BookNameInput
	err := c.ShouldBindJSON(&book_name)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"data": "Invalid input"})
		return
	}

	book_information := services.GetBookInformation(book_name)
	c.JSON(http.StatusOk, gen.H{"data": "OK"})
}
