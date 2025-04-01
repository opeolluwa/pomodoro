package routes

import (
	"com.pomodoro.app/controllers"
	"com.pomodoro.app/repositories"
	"com.pomodoro.app/services"
	"github.com/labstack/echo/v4"
	"gorm.io/gorm"
)

func RegisterRoutes(e *echo.Echo, db *gorm.DB) {
	authResource := e.Group("/v1/auth")
	userRepository := repositories.NewUserUserRepository(*db)
	authenticationService := services.NewAuthenticationService(*userRepository)

	e.GET("/health", controllers.HealthCheck)
	authResource.POST("/register", authenticationService.Register)
}
