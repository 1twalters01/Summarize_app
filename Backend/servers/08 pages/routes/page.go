import (
    "github.com/gin-gonic/gin"
    "net/http"
)

func PageRoutes(router *gin.RouterGroup) {
	router.POST("/author/information", RetrieveAuthorInfo)
	router.POST("/book/information", RetrieveBookInfo)
	router.POST("/pubkisher/information", RetrievePublisherInfo)
	router.POST("/summarizer/information", RetrieveSummryInfo)
	router.POST("/summary/information", RetrieveSummaryInfo)

	// Request to update author information
	// Request to update book information
	// Request to update publisher information
}
