push: commit
	git push origin main

commit: pre-commit
	git add .
	git commit

pre-commit:
	cargo fmt
	cargo test
	cargo deny check
	cargo about generate about.hbs > license.html