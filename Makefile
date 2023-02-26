.PHONY: watch

watch:
	cargo watch -c --ignore .github/ -x 'fmt --check' -x clippy -x test -x run
