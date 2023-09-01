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

ENVS = RUST_LOG=$(LOG_LEVEL) TMPDIR=/mnt/lotus/tmp FIL_PROOFS_PARAMETER_CACHE=/var/tmp/filecoin-proof-parameters LOTUS_WORKER_SKIP_PARAM=true

.PHONY: runcuda
runcuda:
	$(ENVS)  cargo run --features cuda --release -- run

.PHONY: run
run:
	$(ENVS) cargo run -- run