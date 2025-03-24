package utils

import "github.com/Netflix/go-env"

type Environment struct {
	DatabaseHost     string `env:"DATABASE_HOST"`
	DatabaseUser     string `env:"DATABASE_USER"`
	DatabasePort     uint   `env:"DATABASE_PORT"`
	DatabasePassword string `env:"DATABASE_PASSWORD"`
}
