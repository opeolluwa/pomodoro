package config

import (
	"log"

	"github.com/Netflix/go-env"
	"github.com/joho/godotenv"
)

type Environment struct {
	Database struct {
		Port     string `env:"DATABASE_PORT"`
		Name     string `env:"DATABASE_NAME"`
		User     string `env:"DATABASE_USERNAME"`
		Password string `env:"DATABASE_PASSWORD"`
		Host     string `env:"DATABASE_HOST"`
	}
	ServerPort string `env:"PORT"`
	Extras     env.EnvSet
}

func ExtractEnv() *Environment {
	err := godotenv.Load()
	if err != nil {
		log.Fatal(err)
	}
	var environment Environment
	es, err := env.UnmarshalFromEnviron(&environment)
	if err != nil {
		log.Fatal(err)
	}
	environment.Extras = es
	return &environment
}
