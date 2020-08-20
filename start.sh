#!/usr/bin/env bash
# if you get `zsh: permission denied` run `chmod u+x ./start.sh`
# exit when any command fails
set -e

cd server

# build webhook.me
docker build -t captain-hook .

cd ..
cd devops

# run all services in docker
docker-compose up