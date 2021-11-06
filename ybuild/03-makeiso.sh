#!/bin/bash -e

chroot_run() {
	mount_pseudofs
	cp -L /etc/resolv.conf $YDIR/etc/
	chroot $YDIR $@
	retval=$?
	umount_pseudofs
	return $retval
}

mount_pseudofs() {
	mount --bind /dev $YDIR/dev
	mount -t devpts devpts $YDIR/dev/pts -o gid=5,mode=620
	mount -t proc proc $YDIR/proc
	mount -t sysfs sysfs $YDIR/sys
	mount -t tmpfs tmpfs $YDIR/run
}

umount_pseudofs() {
	unmount $YDIR/dev/pts
	unmount $YDIR/dev
	unmount $YDIR/run
	unmount $YDIR/proc
	unmount $YDIR/sys
}

unmount() {
	while true; do
		mountpoint -q $1 || break
		umount $1 2>/dev/null
	done
}

interrupted() {
	die "Abort by user."
}

cleanup() {
	rm -fr $ISODIR
}

die() {
	[ "$@" ] && printerror $@
	umount_pseudofs
	cleanup
	exit 1
}

printstep() {
	echo -e "\e[0;36m::\e[0m $*"
}

printerror() {
	echo -e "\e[0;31mERROR:\e[0m $*"
}

if [ $(id -u) != 0 ]; then
	echo "$0 script need to run as root!"
	exit 1
fi

CWD=$PWD

if [ -f $CWD/config ]; then
	source $CWD/config
fi

ISODIR="$CWD/iso"
LABEL="YANTRALIVECD"
OUTPUT="yantra-livecd-$(date +"%Y%m%d").iso"
FILEDIR="$CWD/files"

#LFS="${LFS:-$CWD/lfs-rootfs}"
YDIR=/mnt/yantra
PKGDIR="${PKGDIR:-$CWD/pkg}"
SRCDIR="${SRCDIR:-$CWD/src}"
WORKDIR="${WORKDIR:-$CWD/work}"

isolinux_files="chain.c32 isolinux.bin ldlinux.c32 libutil.c32 reboot.c32 menu.c32 libcom32.c32 poweroff.c32"

rm -fr $ISODIR
mkdir -p $ISODIR

printstep "Preparing isolinux files..."
mkdir -p $ISODIR/{yantra,isolinux,boot}
for file in $isolinux_files; do
	cp $YDIR/usr/share/syslinux/$file $ISODIR/isolinux || die "failed copying '$file'"
done
#cp isolinux/splash.png $ISODIR/isolinux
cp $FILEDIR/isolinux.cfg $ISODIR/isolinux
[ -d rootfs ] && cp -Ra rootfs $ISODIR

printstep "Make squash filesystem..."
mksquashfs $YDIR $ISODIR/yantra/root.sfs \
		-b 1048576 -comp xz -Xdict-size 100% \
		-e $YDIR/yantra/sources/* \
		-e $YDIR/yantra/packages/* \
		-e $YDIR/yantra/cache/* \
		-e $YDIR/tools/ \
		-e $YDIR/tmp/* 2>/dev/null || die "failed create squashed filesystem"

printstep "Preparing kernel and initramfs..."
cp $YDIR/boot/vmlinuz-yantra $ISODIR/boot/vmlinuz || die "failed copying kernel"
cp files/livecd.hook $YDIR/etc/mkinitramfs.d
kernver=$(file $YDIR/boot/vmlinuz-yantra | cut -d ' ' -f9)
chroot_run mkinitramfs -k $kernver -a livecd -o /boot/initrd-live.img || die "failed create initramfs"
mv $YDIR/boot/initrd-live.img $ISODIR/boot/initrd || die "failed copying initrd"

printstep "Setup UEFI mode..."
mkdir -p $ISODIR/boot/grub/{fonts,x86_64-efi}
mkdir -p $ISODIR/efi/boot
if [ -f $YDIR/usr/share/grub/unicode.pf2 ];then
	cp $YDIR/usr/share/grub/unicode.pf2 $ISODIR/boot/grub/fonts
fi
if [ -f $ISODIR/isolinux/splash.png ]; then
	cp $ISODIR/isolinux/splash.png $ISODIR/boot/grub/
fi
echo "set prefix=/boot/grub" > $ISODIR/boot/grub-early.cfg
cp -a $YDIR/usr/lib/grub/x86_64-efi/*.{mod,lst} $ISODIR/boot/grub/x86_64-efi || die "Failed copying efi files"
cp $FILEDIR/grub.cfg $ISODIR/boot/grub/

grub-mkimage -c $ISODIR/boot/grub-early.cfg -o $ISODIR/efi/boot/bootx64.efi -O x86_64-efi -p "" iso9660 normal search search_fs_file
modprobe loop
dd if=/dev/zero of=$ISODIR/boot/efiboot.img count=4096
mkdosfs -n YDIRUEFI $ISODIR/boot/efiboot.img || die "Failed mkdosfs"
mkdir -p $ISODIR/boot/efiboot
mount -o loop $ISODIR/boot/efiboot.img $ISODIR/boot/efiboot || die "Failed mount efiboot.img"
mkdir -p $ISODIR/boot/efiboot/EFI/boot
cp $ISODIR/efi/boot/bootx64.efi $ISODIR/boot/efiboot/EFI/boot
umount $ISODIR/boot/efiboot
rm -fr $ISODIR/boot/efiboot

printstep "Making the iso..."
rm -f $OUTPUT
xorriso -as mkisofs \
    -isohybrid-mbr $YDIR/usr/share/syslinux/isohdpfx.bin \
    -c isolinux/boot.cat \
    -b isolinux/isolinux.bin \
    -no-emul-boot \
    -boot-load-size 4 \
    -boot-info-table \
    -eltorito-alt-boot \
    -e boot/efiboot.img \
    -no-emul-boot \
    -isohybrid-gpt-basdat \
    -volid $LABEL \
    -o $OUTPUT $ISODIR || die "failed create iso"

printstep "Cleaning up..."
cleanup

exit 0
