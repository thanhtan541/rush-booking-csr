# cargo install cargo-watch
dev:
	cargo watch -x check -x test -x run

fmt:
	cargo fmt
	# cargo install leptosfmt
	leptosfmt ./**/*.rs

check: fmt
	cargo check

PATTERN?="update_db"
test: check
	cargo test ${PATTERN}

test-verbose: check
	cargo test -- --nocapture

# cargo install cargo-tarpaulin
cov:
	cargo tarpaulin --ignore-tests

# rustup component add clippy
lint-check: check
	cargo clippy -- -D warnings

# rustup component add rustfmt
fmt-check:
	cargo fmt -- --check

# cargo install cargo-audit
audit:
	cargo audit

# cargo install cargo-deny
# equivalent to cargo-audit
deny-audit:
	cargo deny

build:
	cargo build

# rustup target add wasm32-unknown-unknown
# cargo install trunk
trunk:
	trunk serve --open

# cargo install cargo-asm
asm:
	cargo asm

scan:
	cargo +nightly udeps

docker-build:
	docker build --tag zero2prod --file Dockerfile .

docker-run:
	docker run -p 8000:8000 zero2prod

# JWT
jwt-keypair:
	./scripts/init_jwt_keypair.sh
