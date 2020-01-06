bundler_docker_image_tag = ruby25-bundler
bundler_dockerfile = misc/ruby25-bundler

pack: clean pack-lambda

clean:
	rm -f *.zip
build-bundler:
	docker build -t $(bundler_docker_image_tag) $(bundler_dockerfile)
pack-lambda: build-bundler
	docker run -it --rm -v $(PWD):/app -v /app/vendor $(bundler_docker_image_tag) sh -c 'cd /app && bundle install --path vendor/bundle --without development && zip -r lambda.zip .'
