SHELL := $(shell which bash)
CONTAINER_NAME = rust:1.40.0-slim-buster
WORKDIR = /workdir

.PHONY: %.pull
%.pull:
	@$* pull ${CONTAINER_NAME}

.PHONY: %.serve
%.serve:
	@$* run --name $@ -p 8000:8000 -v .:${WORKDIR} -w ${WORKDIR} -it ${CONTAINER_NAME} ./init.sh
	@$* rm $@

.PHONY: %.serve.release
%.serve.release:
	@$* run --name $@ -p 8000:8000 -v .:${WORKDIR} -w ${WORKDIR} -it ${CONTAINER_NAME} ./init.sh --release
	@$* rm $@

.PHONY: %.run-bash
%.run-bash:
	-@$* run --name $@ -v .:${WORKDIR} -w ${WORKDIR} -it ${CONTAINER_NAME} bash
	@$* rm $@
