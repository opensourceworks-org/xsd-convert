.PHONY: run build  docker test docker-with-tests

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
	