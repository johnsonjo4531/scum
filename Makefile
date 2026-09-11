go:
	cargo run

vibe:
	jai -j opencode opencode

commit:
	git commit -F ./CURRENT_COMMIT_MESSAGE.md
