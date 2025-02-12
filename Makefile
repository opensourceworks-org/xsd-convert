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

docker:
	@echo "building version: $(VERSION)"
	npm run build:css
	@templ generate
	GOARCH=$(TARGET_ARCH) GOOS=$(TARGET_OS) CGO_ENABLED=0 go build  -o $(DIST)/markitlens
	docker build -t $(APP):$(VERSION) .

docker-run: docker
	docker run -it --rm -p 4444:4444 $(APP):$(VERSION)

docker-deploy: test docker
	@docker tag $(APP):$(VERSION) $(DOCKERHOST)/$(APP):$(VERSION)
	@docker push $(DOCKERHOST)/$(APP):$(VERSION)
	@docker-compose down
	@docker-compose up -d

docker-run-with-tests: test docker-run