#!/usr/bin/env bash
# if you get `zsh: permission denied` run `chmod u+x ./start.sh`
# exit when any command fails
set -e

# remove dangling images
docker system prune

# build webhook.me
cd server
docker build -t captain-hook .

cd ..

# run all services in docker
cd devops
docker-compose up