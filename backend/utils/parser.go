package utils

import "strings"

func ParseString(s string) string {
	return strings.TrimSpace(strings.ToLower(s))
}
