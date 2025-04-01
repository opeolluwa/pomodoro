package main

import (
	"fmt"

	"com.pomodoro.app/config"
	"com.pomodoro.app/routes"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

func main() {
	env := config.ExtractEnv()
	port := env.ServerPort

	if port == "" {
		port = "8080"
	}

	e := echo.New()
	e.Use(middleware.CORS())
	e.Use(middleware.LoggerWithConfig(middleware.LoggerConfig{
		Format: "method=${method}, uri=${uri}, status=${status}\n",
	}))

	db := config.LoadDatabase()
	routes.RegisterRoutes(e, db)

	e.Logger.Fatal(e.Start(fmt.Sprintf(":%v", port)))
}
