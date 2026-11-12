UPSTREAM_REMOTE := git@github.com:zcash/lightwallet-protocol.git
UPSTREAM_VERSION ?= v0.6.0

# Pull the latest (or a specific) upstream proto tag into the subtree.
# Usage:
#   make update-proto                          # stays on v0.6.0
#   make update-proto UPSTREAM_VERSION=vX.Y.Z  # upgrade to a new tag
update-proto:
	git subtree pull --prefix=lightwallet-protocol \
	  $(UPSTREAM_REMOTE) $(UPSTREAM_VERSION) --squash

# After pulling a new tag, regenerate the Rust bindings.
rebuild-proto:
	cargo build --features rebuild-proto

.PHONY: update-proto rebuild-proto
