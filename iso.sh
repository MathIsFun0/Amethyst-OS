# add cargo command later
cd target/x86_64-unknown-uefi
dd if=/dev/zero of=amethyst-os.dd bs=1048576 count=4
echo "g\nn p\n1\n2048\n+1M\nt 1\n1\nw" | fdisk amethyst-os.dd
losetup -D
losetup -o 1048576 --sizelimit 1048576 loop22 amethyst-os.dd
mkfs.vfat -F 16 -n "EFI System" /dev/loop22
mkdir iso
mount /dev/loop22 iso
mkdir iso/EFI iso/EFI/BOOT
cp debug/amethyst-os.efi iso/EFI/BOOT
mv iso/EFI/BOOT/amethyst-os.efi iso/EFI/BOOT/BOOTX64.EFI
umount iso
rm -rf iso
mkdir /amethyst
mv amethyst-os.dd /amethyst/amethyst-os.img
mkisofs -no-emul-boot -o amethyst-os.iso -e amethyst-os.img /amethyst
rm -rf amethyst
