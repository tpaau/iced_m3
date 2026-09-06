doc-no-all-features:
	cargo doc --no-deps -p iced_m3

doc:
	cargo doc --no-deps -p iced_m3 --all-features

open-doc:
	cargo doc --no-deps -p iced_m3 --all-features --open

test:
	cargo test --workspace
	cargo test --workspace --no-default-features

check:
	cargo fmt --check --all
	just test
	just doc-no-all-features
	just doc
	cargo deny check

loc:
	cloc src/
