#!/usr/bin/env bash

# ACCESS TOKEN
# ./auth_access_token.sh username password

curl --location --request POST 'https://auth.aker.network/auth/realms/aker/protocol/openid-connect/token' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'grant_type=password' \
--data-urlencode 'client_id=app-aker' \
--data-urlencode "username=$1" \
--data-urlencode "password=$2" \
--data-urlencode 'scope=openid'

# In api.aker.network add Authorization header, with value = "Bearer access_token" where access_token is the field returned in the response.
