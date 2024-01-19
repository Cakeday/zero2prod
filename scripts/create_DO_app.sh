#!/usr/bin/env bash

doctl apps create --spec spec.yaml

# I might need to deal with migrating the db once I recreate these resources