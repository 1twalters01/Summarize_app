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

	router.POST("/history/get/cached/all", GetCachedHistory)
	router.POST("/history/get/cached/authors", GetCachedAuthors)
	router.POST("/history/get/cached/books", GetCachedBooks)
	router.POST("/history/get/cached/summaries", GetCachedSummaries)
	router.POST("/history/get/cached/summarizers", GetCachedSummarizers)
	router.POST("/history/get/cached/publishers", GetCachedPublishers)

	router.POST("/history/get", GetHistory)

	// Get last n to m authors
	// Get last n to m books
	// Get last n to m summaries
	// Get last n to m summarizers
	// Get last n to m publishers

	router.POST("/history/get/from_author/books", )
	router.POST("/history/get/from_author/books", )
	router.POST("/history/get/from_publisher/authors", )
	router.POST("/history/get/from_publisher/books", )
	router.POST("/history/get/from_publisher/summaries", )
	router.POST("/history/get/from_summarizer/summaries", )

}
