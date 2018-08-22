VERSION ?= latest

# Make sure the right compiler environment installed
.PHONY: setup
setup:
	rustup target add x86_64-unknown-linux-gnu

# Build the binary
.PHONY: build-release
build-release:
	docker run --rm --user "$(id -u)":"$(id -g)" -v "${PWD}":/usr/src/myapp -w /usr/src/myapp rust:1.28.0 cargo build --release

# Build the Docker image
.PHONY: docker-build
docker-build: build-release
	docker build -t mattfarina/github-label-adder:$(VERSION) .

.PHONY: docker-push
docker-push:
	docker push mattfarina/github-label-adder