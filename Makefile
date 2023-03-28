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