all:
	echo hey

#
# NPM world
#

release:
	npx standard-version

#
# Rust world
#

doc:
	cargo doc --open

#
# Git Hooks
#

clean-git-hooks:
	rm .git/hooks/*

setup-hooks: clean-git-hooks
	ln -f git-hooks/* .git/hooks
