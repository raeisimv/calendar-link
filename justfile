publish:
    just fulltest
    cargo publish

build:
    just fulltest;
    cargo build --release

fulltest:
    just fetch-snapshot
    cargo fmt && cargo test

fetch-snapshot:
    mkdir -p __snapshots__
    cd __snapshots__ && curl -OL https://github.com/AnandChowdhary/calendar-link/raw/refs/heads/master/src/__snapshots__/index.spec.ts.snap
    git add __snapshots__

test:
    cargo fmt && cargo test
