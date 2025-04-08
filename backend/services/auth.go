package services

import (
	"log"
	"net/http"
	"time"

	"com.pomodoro.app/adapters/dto"
	"com.pomodoro.app/adapters/response"
	"com.pomodoro.app/config"
	"com.pomodoro.app/entities"
	"com.pomodoro.app/repositories"
	"com.pomodoro.app/utils"
	"github.com/golang-jwt/jwt/v5"
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
		log.Println(err.Error())
		return ctx.JSON(http.StatusBadRequest, response.NewApiResponse(nil, err.Error()))
	}

	user := s.UserRepository.FindOneByEmail(request.Email)
	if user != (entities.User{}) {
		return ctx.JSON(http.StatusConflict, response.NewApiResponse(nil, "A user with the provided email already exists"))
	}

	_, err := s.UserRepository.Create(*request)
	if err != nil {
		log.Println(err.Error())
		return ctx.JSON(http.StatusInternalServerError, response.NewApiResponse(nil, err.Error()))
	}

	claims := &utils.JwtClaims{
		Identifier: user.Identifier,
		Email:      user.Email,
		RegisteredClaims: jwt.RegisteredClaims{
			ExpiresAt: jwt.NewNumericDate(time.Now().Add(time.Minute * 5)), // 5 mnutes
		},
	}

	// token := jwt.NewWithClaims(jwt.SigningMethodHS512, claims)
	token := jwt.NewWithClaims(jwt.SigningMethodHS512, claims)

	hash, err := token.SignedString([]byte(config.ExtractEnv().Secrets.Jwt))
	println(hash)
	if err != nil {
		log.Fatal(err)
		s.UserRepository.Delete(user.Identifier)
		return ctx.JSON(http.StatusInternalServerError, response.NewApiResponse(nil, "Request could not be processed at this time, please try again later"))
	}
	return ctx.JSON(http.StatusCreated, response.NewApiResponse(response.SignUpResponse{
		Jwt: hash,
	}, "Account successfully created"))
}

// func (s *AuthenticationService) Login(ctx echo.Context) error {

// }

// func (s *AuthenticationService) VerifyEmail(ctx echo.Context) error {

// }

// func (s *AuthenticationService) ForgottenPassword(ctx echo.Context) error {

// }

// func (s *AuthenticationService) ConfirmResetOtp(ctx echo.Context) error {

// }

// func (s *AuthenticationService) SetNewPassword(ctx echo.Context) error {

// }
