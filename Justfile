APP_NAME := "pomodoro"
APP_VERSION :="0.0.1"
MINIMUM_STABLE_RUST_VERSION :="1.83.0"
BINARIES_PATH := "bin"
EXPORT_PATH := "packages"
SUPPORTED_PLATFORM :="android ios"

alias d := develop
alias w:= develop 

set export := true
set dotenv-filename := "./.envrc"
set dotenv-load :=  true 
set ignore-comments := true

default: 
    @just --list --list-heading $'Available commands\n'

prepare:
      python3 -m venv ./scripts/venv
      source ./scripts/venv/bin/activate

develop platform:
    #!/usr/bin/env sh
    export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
    export ANDROID_HOME="$HOME/Library/Android/sdk"
    export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"
    export LEPTOS_TAILWIND_VERSION='v4.0.0'
    sh scripts/develop {{platform}}


run platform:
    #!/usr/bin/env sh
    python3 scripts/run.py {{platform}}


logs platform:
    #!/usr/bin/env sh
        python3 scripts/logger.py {{platform}}

fmt: 
    cd ./pomodoro leptosfmt . && cargo fmt && cargo group-imports --fix && cargo sort -w

