import (
    "github.com/gin-gonic/gin"
    "net/http"
)

func HistoryRoutes(router *gin.RouterGroup) {
	// Add to history
	router.POST("/history/add", AddHistoryItem)

	// delete uuid list from history instead of all the other deletes
	router.POST("/history/delete/uuids", DeleteFromUUID)
	// delete publishers from history
	// delete authors from history
	// delete summarizers from history
	// delete summaries from history
	// delete days from history
	router.POST("/history/delete/days", DeleteFromRange)
	// delete all history
	router.POST("/history/delete/all", DeleteAllHistory)

	// Get detailed view

	// Get authors (cached)
	// Get books (cached)
	// Get summaries (cached)
	// Get sumarizers (cached)
	// Get publishers (cached)

	// Get last n to m authors
	// Get last n to m books
	// Get last n to m summaries
	// Get last n to m summarizers
	// Get last n to m publishers

	// Get last read authors from publisher
	// Get last read books from publisher
	// Get last read summaries from publisher
	// Get last read summaries from summarizer
	// Get last read books from author
	// Get last read summaries from author
}
