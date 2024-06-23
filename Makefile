push: commit
	git push origin main

commit: check
	git add .
	git commit

check:
	cargo check
	cargo fmt
	cargo test
	cargo deny check
	cargo machete
	cargo about generate about.hbs > NOTICE.html