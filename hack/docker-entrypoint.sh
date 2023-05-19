    if [ "$1" = "gctl" ] || [ "$1" = "github-helper" ]; then
        exec "$@"
    else
        sh gctl "$@"
    fi