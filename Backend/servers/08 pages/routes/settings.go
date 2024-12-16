package routes

import (
    "github.com/gin-gonic/gin"
    "net/http"
)

func SettingsRoutes(router *gin.RouterGroup) {
	router.POST("settings/explicit", ToggleExplicitContent)
	router.POST("settings/hide-genres", HideGenres)
    router.POST("settings/show-genres", ShowGenres)
}
