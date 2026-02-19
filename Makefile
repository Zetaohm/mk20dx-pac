DEVICES = mk20d5 mk20d7

.PHONY: all patch svd2rust fmt check clean
.PHONY: $(DEVICES:%=patch-%) $(DEVICES:%=svd2rust-%) $(DEVICES:%=fmt-%) $(DEVICES:%=check-%)

all: patch svd2rust fmt check

# Apply svdtools YAML patches to SVD files
patch: $(DEVICES:%=patch-%)
patch-%:
	svdtools patch devices/$*.yaml

# Generate Rust crate code from patched SVDs
svd2rust: $(DEVICES:%=svd2rust-%)
svd2rust-%:
	@if [ -f svd/$*.svd.patched ]; then \
		cd $* && svd2rust -i ../svd/$*.svd.patched; \
	else \
		cd $* && svd2rust -i ../svd/$*.svd; \
	fi
	cd $* && form -i lib.rs -o src/
	rm -f $*/lib.rs

# Format generated code
fmt: $(DEVICES:%=fmt-%)
fmt-%:
	cd $* && cargo fmt

# Check generated crates compile
check: $(DEVICES:%=check-%)
check-%:
	cd $* && cargo check

# Clean generated files
clean:
	rm -f svd/*.patched
	rm -rf mk20d5/src/*.rs mk20d5/build.rs mk20d5/device.x
	rm -rf mk20d7/src/*.rs mk20d7/build.rs mk20d7/device.x

# Run comparison script
compare-%:
	uv run python3 scripts/compare_header_svd.py --variant $* --svd svd/$*.svd --header reference/kinetis.h
