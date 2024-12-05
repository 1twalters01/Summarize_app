package main

import (
    "github.com/gin-gonic/gin"
)

func main() {
    r := gin.Default()

    config.LoadConfig()

    routes.PagesRoutes(r)

    r.Run(":8008")
}
