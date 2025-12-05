export DIRENV_LOG_FORMAT := ""

run:
    direnv exec $(pwd) cargo run -q

test:
    direnv exec $(pwd) cargo nextest run --cargo-quiet
