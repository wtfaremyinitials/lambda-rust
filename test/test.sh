cargo build
lambda-local -c -l index.handler.js -e test/example_event.json
