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

	router.POST("/history/get/from_author/books", FromAuthorGetBooks)
	router.POST("/history/get/from_author/summaries", FromAuthorGetSummaries)
	router.POST("/history/get/from_publisher/authors", FromPublisherGetAuthors)
	router.POST("/history/get/from_publisher/books", FromPublisherGetBooks)
	router.POST("/history/get/from_publisher/summaries", FromPublisherGetSummaries)
	router.POST("/history/get/from_summarizer/summaries", FromSummarizerGetSummaries)

}
