UID := $(shell id -u $(USER))
GID := $(shell id -g $(USER))

all: bin

bin:
	env UID=$(UID) GID=$(GID) docker compose run --rm rust-centos7;

clean:
	rm build/*.rpm
