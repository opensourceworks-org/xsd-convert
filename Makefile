.PHONY: run build  docker test docker-with-tests latest

#include .env
#export

VERSION := $(shell cat VERSION.txt)
DIST := dist
APP := xsd-convert
TARGET_ARCH := arm64
TARGET_OS := linux

clean:
	rm -rf dist
	rm -rf tmp

run:
	@trunk serve

build:
	@trunk build --release


release:
	@curl -L -o build.zip https://github.com/opensourceworks-org/xsd-convert/releases/download/latest/build.zip
	@rm -rf /container_storage/openresty/var/www/xsd-convert.com/app/*
	@unzip build.zip -d /container_storage/openresty/var/www/xsd-convert.com/app/

latest:
	git tag -f latest -m "chore: release latest"
	git push origin latest --force