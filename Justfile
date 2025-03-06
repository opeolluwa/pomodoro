APP_NAME := "pomodoro"
APP_VERSION :="0.0.1"
MINIMUM_STABLE_RUST_VERSION :="1.83.0"
BINARIES_PATH := "bin"
EXPORT_PATH := "packages"
SUPPORTED_PLATFORM :="android ios"

alias d := develop

set export := true
set dotenv-filename := ".envrc"
set dotenv-load :=  true 
set ignore-comments := true

default: 
    @just --list --list-heading $'Available commands\n'

prepare:
      python3 -m venv ./scripts/venv
      source ./scripts/venv/bin/activate

develop platform:
    python3 scripts/develop.py {{platform}}