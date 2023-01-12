.PHONY: build-node
build-node:
	cargo build --release -p parachain-template-node

.PHONY: build-polkadot-parachain
build-polkadot-parachain:
	cargo build --release -p polkadot-parachain

.PHONY: build
build:
	cargo build --release

.PHONY: update
update:
	cargo update