all:
	echo hey

doc:
	cargo doc --open

clean-git-hooks:
	rm .git/hooks/*

setup-hooks: clean-git-hooks
	ln -f git-hooks/* .git/hooks
