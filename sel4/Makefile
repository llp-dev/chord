IMAGE := chord/sel4-builder

.PHONY: image-build
image-build:
	podman build -t $(IMAGE) .

.PHONY: kernel
kernel: config.cmake
	podman run --rm \
		-v $(PWD):/sel4:Z \
		-v $(PWD)/config.cmake:/tmp/config.cmake:Z \
		$(IMAGE)
