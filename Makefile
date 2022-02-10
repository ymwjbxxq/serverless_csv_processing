FUNCTIONS := handler
STACK_NAME := rust-csv-processig
ARCH := aarch64-unknown-linux-gnu


build:
	rm -rf ./build
	cross build --release --target $(ARCH)
	mkdir -p ./build
	${MAKE} ${MAKEOPTS} $(foreach function,${FUNCTIONS}, build-${function})

build-%:
	mkdir -p ./build/$*
	cp -v ./target/$(ARCH)/release/$* ./build/$*/bootstrap

deploy:
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-s3 					--template-file ./deployment/s3-template.yml
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-eventbridge	--template-file ./deployment/eventbridge-template.yml
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name ${STACK_NAME}-lambda 			--template-file ./deployment/lambda-template.yml

delete:
	sam delete --profile test --stack-name ${STACK_NAME}-eventbridge
	sam delete --profile test --stack-name ${STACK_NAME}-lambda
	sam delete --profile test --stack-name ${STACK_NAME}-s3