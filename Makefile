.PHONY: build-dos2unix build-gzip-rust build-htop-unix-rust build-htop-win-rust \
	build-gzip-go build-htop-win-go build-rust build-go \
	test-rust test-go test-all

build-dos2unix:
	cargo build --release --manifest-path dos2unix/Cargo.toml

build-gzip-rust:
	cargo build --release --manifest-path gzip/rust/Cargo.toml

build-htop-unix-rust:
	cargo build --release --manifest-path htop/unix/rust/Cargo.toml

build-htop-win-rust:
	cargo build --release --manifest-path htop/win/rust/Cargo.toml

build-gzip-go:
	$(MAKE) -C gzip/go build

build-htop-win-go:
	cd htop/win/go && go build ./...

build-rust: build-dos2unix build-gzip-rust build-htop-unix-rust build-htop-win-rust

build-go: build-gzip-go build-htop-win-go

test-rust:
	cargo fmt --all
	cargo clippy --all-targets -- -D warnings
	cargo test --all --all-features

test-go:
	gofmt -w .
	go vet ./...
	go test ./...

test-all: test-rust test-go
