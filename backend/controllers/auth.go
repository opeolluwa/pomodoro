package controllers


import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func Register(c echo.Context) error {
	return c.String(http.StatusOK, "Service is healthy")
}
