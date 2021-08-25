obj-m += syscall_hook.o
syscall_hook-objs += driver.o target/x86_64-unknown-none-linuxkernel/release/liblogger.a

all:
	cargo xbuild -Z build-std=core,alloc --target=x86_64-unknown-none-linuxkernel --release
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	# cargo clean
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
