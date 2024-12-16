package routes

import (
    "github.com/gin-gonic/gin"
    "net/http"
)

func PingRoutes(router *gin.RouterGroup) {
	router.POST("/ping/health", Ping)
	router.POST("/ping/author/information", ExampleAuthorInfo)
	router.POST("/ping/book/information", ExampleBookInfo)
	router.POST("/ping/publisher/information", ExamplePublisherInfo)
	router.POST("/ping/summarizer/information", ExampleSummryInfo)
	router.POST("/ping/summary/information", ExampleSummaryInfo)
}
