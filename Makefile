.PHONY: fmt
fmt:
	cargo fix --allow-dirty
	cargo fmt