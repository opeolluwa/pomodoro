#!/usr/bin/env sh

APP_NAME := "pomodoro"
APP_VERSION :="0.0.1"
MINIMUM_STABLE_RUST_VERSION :="1.83.0"
BINARIES_PATH := "release"
EXPORT_PATH := "packages"
SUPPORTED_PLATFORM :="android ios"
SCRIPTS_PATH := "./scripts"

alias d := develop
alias w:= develop 


set export := true
set dotenv-filename := "./backend/.env"
set dotenv-load :=  true 
set ignore-comments := true

default: 
    @just --list --list-heading $'Available commands\n'

develop platform:
    #!/usr/bin/env sh
    sh {{SCRIPTS_PATH}}/develop $(sh {{SCRIPTS_PATH}}/platform  {{platform}})
fmt: 
    leptosfmt .
    cargo fmt --manifest-path=pomodoro/Cargo.toml
    cargo group-imports --fix -- --manifest-path=pomodoro/Cargo.toml
    cargo sort -w --  --manifest-path=pomodoro/Cargo.toml
ship:
    #!/bin/bash 
    cp app/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk {{BINARIES_PATH}}/{{APP_NAME}}.apk
    cp app/src-tauri/gen/android/app/build/outputs/bundle/universalRelease/app-universal-release.aab {{BINARIES_PATH}}/{{APP_NAME}}.aab

