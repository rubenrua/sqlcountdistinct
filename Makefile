all:
	@echo 'SQL COUNT DISTINCT'
	@echo '=================='
	@echo ''
	@echo 'make release'
	@echo 'make test'
	@echo 'make fmt'
	@echo 'make ci'
release:
	cargo build --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/sqlcountdistinct target/
	strip dist/sqlcountdistinct
test:
	cargo test
fmt:
	cargo fmt
ci:
	cargo fmt -- --check
	touch ./src/*.rs && cargo clippy
	cargo build
	cargo test

