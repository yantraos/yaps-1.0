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

# toolchain
for i in  01-binutils-tc-pass1 \
          02-gcc-tc-pass1 \
          03-linux-api-headers-tc \
          04-glibc-tc \
          05-libstdc++-tc-pass1 \
          06-m4-tc \
          07-ncurses-tc \
          08-bash-tc \
          09-coreutils-tc \
          10-diffutils-tc \
          11-file-tc \
          12-findutils-tc \
          13-gawk-tc \
          14-grep-tc \
          15-gzip-tc \
          16-make-tc \
          17-patch-tc \
          18-sed-tc \
          19-tar-tc \
          20-xz-tc \
          21-binutils-tc-pass2 \
          22-gcc-tc-pass2 \
          23-yaps-tc \
          24-openssl-tc \
          25-wget-tc; do
    [[ -e $YDIR/tools/installed/$i ]] && {
        echo "skipping $i, already configured"
        continue
    }
    echo "=> compiling $i"
    echo -e '\033k' compiling $i '\033\\'
    yaps compile $i --no-install --no-depends --compiler-specs $COMPILER_SPECS
    if [[ $? != 0 ]] ; then
        echo "ERROR failed to compile $i"
        exit 1
    fi

    touch $YDIR/tools/installed/$i
done

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

for i in chroot-01-filesystem-tc \
         chroot-02-libstdc++-tc-pass2 \
         chroot-03-gettext-tc \
         chroot-04-bison-tc \
         chroot-05-perl-tc \
         chroot-06-python-tc \
         chroot-07-texinfo-tc \
         chroot-08-util-linux-tc; do

    [[ -e $YDIR/tools/installed/$i ]] && {
      echo "skipping $i, already configured"
      continue
    }

    echo "=> compiling $i"
    echo -e '\033k' compiling $i '\033\\'

    ychroot /tools/bin/yaps compile $i --no-install --no-depends --compiler-specs /yaps.build.conf

    touch $YDIR/tools/installed/$i
done

ychroot /tools/bin/yaps compile chroot-clean-tc --no-install --no-depends --compiler-specs /yaps.build.conf

strip --strip-debug $YDIR/usr/lib/*
strip --strip-unneeded $YDIR/usr/{,s}bin/*
strip --strip-unneeded $YDIR/tools/bin/*
