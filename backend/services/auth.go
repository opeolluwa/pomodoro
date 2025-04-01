package services

import (
	"log"
	"net/http"

	"com.pomodoro.app/adapters/dto"
	"com.pomodoro.app/adapters/response"
	"com.pomodoro.app/repositories"
	"github.com/labstack/echo/v4"
)

type AuthenticationService struct {
	UserRepository repositories.UserRepository
}

func NewAuthenticationService(repo repositories.UserRepository) *AuthenticationService {
	return &AuthenticationService{
		UserRepository: repo,
	}
}

func (s *AuthenticationService) Register(ctx echo.Context) error {
	request := new(dto.CreateUserDto)
	if err := ctx.Bind(request); err != nil {
		return err
	}
	

	// user := s.UserRepository.FindOneByEmail(request.Email)
	// if user != entities.User{} {
	// 	return ctx.JSON(http.StatusConflict, reponse.NewApiResponse(nil, "A user with the provided email already exists"))
	// }

	_, err := s.UserRepository.Create(*request)
	if err != nil {
		log.Fatal(err)
		// return ctx.JSON(http.StatusInternalServerError, response.NewApiResponse(nil, err.Error()))
	}


	return ctx.JSON(http.StatusCreated, response.NewApiResponse(nil, "Account successfully created"))
}
