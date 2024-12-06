import (
    "github.com/gin-gonic/gin"
    "net/http"
)

func HistoryRoutes(router *gin.RouterGroup) {
	router.POST("/history/add", AddHistoryItem)

	router.POST("/history/delete/uuids", DeleteFromUUIDs)
	router.POST("/history/delete/publishers", DeleteFromPublishers)
	router.POST("/history/delete/authors", DeleteFromAuthors)
	router.POST("/history/delete/sumarizers", DeleteFromSummarizes)
	router.POST("/history/delete/summaries", DeleteFromSummaries)
	router.POST("/history/delete/days", DeleteFromRange)
	router.POST("/history/delete/all", DeleteAllHistory)

	// Get all (cached)
	router.POST("/history/get/cached/all", )
	// Get authors (cached)
	router.POST("/history/get/cached/authors", )
	// Get books (cached)
	router.POST("/history/get/cached/books", )
	// Get summaries (cached)
	router.POST("/history/get/cached/summaries", )
	// Get sumarizers (cached)
	router.POST("/history/get/cached/summarizers", )
	// Get publishers (cached)
	router.POST("/history/get/cached/publishers", )

	// Get last n to m authors
	// Get last n to m books
	// Get last n to m summaries
	// Get last n to m summarizers
	// Get last n to m publishers

	router.POST("/history/get", GetHistory)

	// Get last read books from author
	router.POST("/history/get/from_author/books", )
	// Get last read summaries from author
	router.POST("/history/get/from_author/books", )
	// Get last read authors from publisher
	router.POST("/history/get/from_publisher/authors", )
	// Get last read books from publisher
	router.POST("/history/get/from_publisher/books", )
	// Get last read summaries from publisher
	router.POST("/history/get/from_publisher/summaries", )
	// Get last read summaries from summarizer
	router.POST("/history/get/from_summarizer/summaries", )

}
