push: commit
	git push origin main

commit: check
	git add .
	git commit

check: unsafe_check
	cargo check
	cargo fmt
	cargo test
	cargo deny check
	cargo machete
	cargo about generate ./scripts/about.hbs > NOTICE.html

unsafe_check:
	find . -type f -name "*.rs" -not -path "./target/*" -exec ./scripts/checkunsafe.py {} \;
