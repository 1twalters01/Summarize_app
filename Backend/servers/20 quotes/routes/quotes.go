import (
    "github.com/gin-gonic/gin"
    "net/http"
)

func PageRoutes(router *gin.RouterGroup) {
	router.GET("/quotes/fetch-all", RetrieveAllQuotes)
	router.POST("/quotes/fetch", RetrieveSpecificQuotes)
	router.POST("/quotes/add", CreateNewQuote)
	router.POST("/quotes/delete", DeleteQuote)
	router.POST("/quotes/modify", ModifyQuote)
}
