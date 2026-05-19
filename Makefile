NAME        ?= kfs
TOOLS_DIR   := tools
DOCKER_TAG  := kfs-builder

DOCKER_CMD  := docker run --rm -v "$(shell pwd):/root/env:Z" -it $(DOCKER_TAG)
DOCKER_RUN  := docker run --rm -v "$(shell pwd):/root/env:Z" -p 5900:5900 -it $(DOCKER_TAG)

all: iso run

build:
	@$(DOCKER_CMD) $(TOOLS_DIR)/build.sh

$(NAME).iso: build
	@$(DOCKER_CMD) $(TOOLS_DIR)/iso.sh

iso: $(NAME).iso

run: $(NAME).iso
	@$(DOCKER_RUN) $(TOOLS_DIR)/run.sh

image:
	docker build -t $(DOCKER_TAG) .

clean:
	rm -rf build sysroot target $(NAME).iso

fclean:
	rm -rf build sysroot target $(NAME).iso 
	podman rmi localhost/kfs-builder 

re: clean all

.PHONY: image build iso run clean re