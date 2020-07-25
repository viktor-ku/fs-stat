all:
	echo hey

#
# NPM world
#

release:
	npx standard-version --no-verify

ci/build:
	npx ncc build ci/src/cargo-toml.ts -o ci/dist/cargo-toml/ & \
		npx ncc build ci/src/cargo-toml-lock.ts -o ci/dist/cargo-toml-lock/

ci/clean:
	rm -rf ci/dist

ci/install:
	npm ci

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
