.PHONY: default build test clean run run_without_network build_release build_amd64_static docker_build_amd64_static release_amd64_static build_armv7_static docker_build_armv7_static release_armv7_static release_with_docker_only release

amd64_target := x86_64-unknown-linux-musl
arm32v7_target := armv7-unknown-linux-musleabihf
default: release_static_arm

build:
	cargo build

test:
	cargo test

clean:
	cargo clean

run:
	cargo run

run_without_network:
	unshare -r -n -- cargo run

build_release:
	cargo build --release

build_amd64_static:
	cross build --release --target $(amd64_target)

docker_build_amd64_static:
	mkdir -p target/output
	cp target/$(amd64_target)/release/bulkmail target/output/
	VERSION=$$(./target/output/bulkmail --version | cut -f2 -d ' '); \
	docker buildx build -t giggio/bulkmail:$$VERSION-amd64 -t giggio/bulkmail:amd64 --platform linux/amd64 --build-arg PLATFORM=x86_64 --push .

release_amd64_static: build_amd64_static docker_build_amd64_static

build_armv7_static:
	cross build --release --target $(arm32v7_target)

docker_build_armv7_static:
	mkdir -p target/output
	cp target/$(arm32v7_target)/release/bulkmail target/output/
	VERSION=$$(./target/output/bulkmail --version | cut -f2 -d ' '); \
	docker buildx build -t giggio/bulkmail:$$VERSION-arm32v7 -t giggio/bulkmail:arm32v7 --platform linux/arm/v7 --build-arg PLATFORM=armhf --push .

release_armv7_static: build_armv7_static docker_build_armv7_static

release_with_docker_only: docker_build_amd64_static docker_build_armv7_static
	DOCKER_CLI_EXPERIMENTAL=enabled docker manifest create giggio/bulkmail:latest \
		--amend giggio/bulkmail:amd64 \
		--amend giggio/bulkmail:arm32v7
	DOCKER_CLI_EXPERIMENTAL=enabled docker manifest push giggio/bulkmail:latest
	VERSION=$$(./target/$(amd64_target)/release/bulkmail --version | cut -f2 -d ' '); \
	DOCKER_CLI_EXPERIMENTAL=enabled docker manifest create giggio/bulkmail:$$VERSION \
		--amend giggio/bulkmail:$$VERSION-amd64 \
		--amend giggio/bulkmail:$$VERSION-arm32v7; \
	DOCKER_CLI_EXPERIMENTAL=enabled docker manifest push giggio/bulkmail:$$VERSION

release: release_amd64_static release_armv7_static release_with_docker_only
