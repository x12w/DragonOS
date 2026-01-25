ifeq ($(ARCH), x86_64)
	CROSS_COMPILE=x86_64-linux-musl-
else ifeq ($(ARCH), riscv64)
	CROSS_COMPILE=riscv64-linux-musl-
endif

CC=$(CROSS_COMPILE)gcc


all: syscall_test.c
	$(CC) -static -o syscall_test syscall_test.c

.PHONY: install clean
install: all
	mv about $(DADK_CURRENT_BUILD_DIR)/syscall_test.elf

clean:
	rm about *.o sys_version.h

fmt:
