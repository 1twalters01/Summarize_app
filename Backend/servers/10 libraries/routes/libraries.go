import (
    "github.com/gin-gonic/gin"
    "net/http"
)

func LibrariesRoutes(router *gin.RouterGroup) {
	// Get cached summaries
	// Get library information
	// Get all summaries

	// Add to library
	router.POST("/library/add", AddToLibrary)

	// Remove from library
	router.POST("/library/remove", RemoveFromLibrary)
	// Remove all library
	router.POST("/library/remove/all", RemoveAllFromLibrary)

	// Reorder library
	router.POST("/library/", ReorderLibrary)

	// Toggle privacy status of library
}