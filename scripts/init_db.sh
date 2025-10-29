#!/usr/bin/env bash
set -x
set -eo pipefail

#check if custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
#check is a custom password has been set, otherwise default to 'staple'
DB_PASSWORD="${POSTGRES_PASSWORD:=staple}"
#check if custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
#check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"
#check if custom host is set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

#launch Postgres using Docker
