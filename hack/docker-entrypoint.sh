#!/usr/bin/env bash

if [ "$1" = "gctl" ]; then
    exec "$@"
else
    sh -c gctl "$@"
fi