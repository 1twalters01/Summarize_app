package routes

import (
    "github.com/gin-gonic/gin"
    "net/http"
)

func SettingsRoutes(router *gin.RouterGroup) {
	router.POST("/settings/history/record-history", ToggleHistory)
	router.POST("/settings/history/duration", HistoryDuration)
	router.POST("/settings/history/share", HistoryShareStatus)
}
