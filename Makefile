LOG_LEVEL=debug

.PHONY: fmt
fmt:
	cargo fix --allow-dirty
	cargo fmt
	cd fil-proofs-param && cargo fix --allow-dirty
	cd fil-proofs-param && cargo fmt

.PHONY: check
check:
	cargo +nightly udeps --all-targets
	cargo fmt --check

.PHONY: run
run:
	RUST_LOG=$(LOG_LEVEL) LOTUS_WORKER_SECTOR_SIZE=2KiB cargo run -- run