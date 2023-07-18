LOG_LEVEL=info
# LOG_LEVEL=trace

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

ENVS = RUST_LOG=$(LOG_LEVEL) FIL_PROOFS_PARAMETER_CACHE=~/.lotusworker/filecoin-proof-parameters LOTUS_WORKER_SKIP_PARAM=true LOTUS_WORKER_SECTOR_SIZE=2KiB

.PHONY: runcuda
runcuda:
	$(ENVS)  cargo run --features cuda -- run

.PHONY: run
run:
	$(ENVS) cargo run -- run