#!/usr/bin/env bash

# REFRESH TOKEN
# ./auth_refresh_token.sh refresh_token

curl --location --request POST 'https://auth.aker.network/auth/realms/aker/protocol/openid-connect/token' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'grant_type=refresh_token' \
--data-urlencode 'client_id=app-aker' \
--data-urlencode "refresh_token=$1" \
--data-urlencode 'scope=openid'
