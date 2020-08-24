#!/bin/bash

BASE_PATH=$(dirname $0)
docker-compose exec roach1 cockroach sql --insecure --host=localhost:26257 -d review --execute="$(cat ${BASE_PATH}/sql/create_db.sql)"
