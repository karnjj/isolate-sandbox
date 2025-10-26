docker-build:
	docker buildx build --platform linux/amd64 -t isolate-sandbox -f Dockerfile .

docker-run:
	docker run --privileged -p 3000:3000 isolate-sandbox