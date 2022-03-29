# Amethyst-OS
Trying to make our own, open-source operating system. Will this project die like the rest that I've tried?

Currently in-development. We're just trying to get this to work. :)

# Installation Instructions
## QEMU, Alpha v0.0.1 - ???
Download [OVMF.fd](https://github.com/clearlinux/common/blob/master/OVMF.fd).
To run the program, you must extract the ZIP file and run the command `qemu-system-x86_64 -drive if=pflash,format=raw,file=C:/path/to/OVMF.fd -drive file=fat:rw:C:/path/to/amethyst-os/debug,format=raw` to start the shell. Once the shell finishes booting, run `amethyst-os.efi`.
