[private]
default:
    @just --list

run-client:
    dx serve -p client

run-web:
    dx serve -p client --web
