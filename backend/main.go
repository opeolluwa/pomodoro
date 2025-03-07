package main

import (
	"fmt"
	"net/http"
	"pomodoro/utils"

	"github.com/labstack/echo/v4"
)

func main() {
	port := utils.GetEnvParameter("PORT")
	e := echo.New()
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, " Hello world")
	})
	e.Logger.Fatal(e.Start(fmt.Sprintf(":%v", port )))
}
