package utils

import "os"

func GetEnvParameter(key string) string {
	env := os.Getenv(key)
	return env
}
