#!/usr/bin/env bash

gq https://api.roboxmaker.com/v1/graphql -H "X-Hasura-Admin-Secret: a1k8e3r4" --introspect > crates/roboxmaker_models/schema.graphql
