#!/usr/bin/env sh

if [ "$1" = "gctl" ]; then
    exec "$@"
else
    gctl "$@"
fi