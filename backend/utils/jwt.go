package utils

import "github.com/golang-jwt/jwt/v5"

type JwtClaims struct {
	Identifier string `json:"identifier"`
	Email      string `json:"email"`
	jwt.RegisteredClaims
}
