package routes

import (
	"com.pomodoro.app/controllers"
	"github.com/labstack/echo/v4"
	"gorm.io/gorm"
)

func RegisterRoutes(e *echo.Echo, db *gorm.DB) {
	e.GET("/health", controllers.HealthCheck)
}
