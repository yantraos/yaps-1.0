COMPILER_SPECS=$(pwd)/yaps.build.conf
source $COMPILER_SPECS

export SRCDIR=$YDIR/yantra/sources
export PKGDIR=$YDIR/yantra/packages
export WORKDIR=$YDIR/yantra/cache
CFLAGS="${CFLAGS:--O2 -march=x86-64 -pipe}"

mkdir -p $SRCDIR $PKGDIR $WORKDIR

mkdir -pv $YDIR/{bin,etc,lib,sbin,usr,var}
case $(uname -m) in
  x86_64) mkdir -pv $YDIR/lib64 ;;
esac

[[ -d $TCDIR ]] || mkdir -p $TCDIR

export REPOSITORY=$(pwd)/

if [ ! -L /tools ]; then
	ln -svf $YDIR/tools /tools
fi

[[ -d $YDIR/tools/installed ]] || mkdir -p $YDIR/tools/installed

export PATH=$YDIR/tools/bin:/bin:/usr/bin:$PATH

[[ -e $YDIR/yantra/yantra-kernel-fs ]] || {
    mkdir -pv $YDIR/{dev,proc,sys,run}

    mknod -m 600 $YDIR/dev/console c 5 1
    mknod -m 666 $YDIR/dev/null c 1 3

    touch $YDIR/yantra/yantra-kernel-fs
}

ychroot() {
    [[ -d $YDIR/yantra/repository ]] || mkdir -p $YDIR/yantra/repository

    _mount() {
        mount -v --bind /dev $YDIR/dev
        mount -vt devpts devpts $YDIR/dev/pts -o gid=5,mode=620
        mount -vt proc proc $YDIR/proc
        mount -vt sysfs sysfs $YDIR/sys
        mount -vt tmpfs tmpfs $YDIR/run
        if [ -h $YDIR/dev/shm ]; then
          mkdir -pv $YDIR/$(readlink $YDIR/dev/shm)
        fi

        mount --bind $(realpath .) $YDIR/yantra/repository
    }

    _umount() {
        umount $YDIR/yantra/repository
        umount $YDIR/dev{/pts,}
        umount $YDIR/{sys,proc,run}
    }

    # mount pseudo filesystem
    _mount

    echo "executing $@"
    # chroot command
    chroot "$YDIR" /tools/bin/env -i \
    HOME=/root \
    TERM="$TERM" \
    PS1='toolchain (yantra) \u:\w \$ ' \
    PATH=/bin:/usr/bin:/sbin:/usr/sbin:/tools/bin \
    $@

    retval=$?

    sleep 1
    # un-mount
    _umount

    if [[ "$retval" != "0" ]] ; then
        echo "Failed to do '$@'"
        exit 1
    fi

    return $retval
}

cp yaps.build.conf $YDIR/

for i in man-pages \
         tcl \
         expect \
         dejagnu \
         iana-etc \
         glibc \
         zlib \
         bzip2 \
         xz \
         zstd \
         file \
         readline \
         m4 \
         bc \
         flex \
         binutils \
         gmp \
         mpfr \
         mpc \
         attr \
         acl \
         libcap \
         shadow \
         gcc \
         pkg-config \
         ncurses \
         sed \
         psmisc \
         gettext \
         bison \
         grep \
         bash \
         libtool \
         gdbm \
         gperf \
         expat \
         inetutils \
         perl \
         XML-Parser \
         intltool \
         autoconf \
         automake \
         kmod \
         elfutils \
         libffi \
         openssl \
         Python \
         ninja \
         meson \
         coreutils \
         check \
         diffutils \
         gawk \
         findutils \
         groff \
         grub \
         less \
         gzip \
         iproute2 \
         kbd \
         libpipeline \
         make \
         patch \
         man-db \
         tar \
         texinfo \
         vim \
         dbus \
         procps-ng \
         util-linux \
         e2fsprogs \
         sysklogd \
         sysvinit \
         eudev \
         lfs-bootscripts \
         linux-firmware \
         libarchive \
         mkinitramfs \
         libtasn1 \
         p11-kit \
         wget \
         pkgutils \
         libnl \
         libxml2 \
         dhcpcd \
         wpa_supplicant \
         gpm \
         blfs-bootscripts \
         lzo \
         lz4 \
         squashfs-tools \
         popt \
         efivar \
         pciutils \
         efibootmgr \
         libpng \
         which \
         freetype \
         dosfstools \
         grub-efi \
         libevent \
         links \
         mtools \
         syslinux \
         ca-certificates \
         mtools \
         curl \
         fuse \
         yaps \
         linux; do

    [[ -d $YDIR/usr/share/yaps/$i ]] && {
      echo "skipping $i, already configured"
      continue
    }

    echo "=> compiling $i"

    ychroot /tools/bin/yaps compile $i --no-depends --compiler-specs yaps.build.conf
done

for i in 01-sysconfig \
         02-strip \
         03-clean; do

    echo "=> compiling $i"
    ychroot /tools/bin/yaps compile $i --no-install --no-depends --compiler-specs yaps.build.conf
done
