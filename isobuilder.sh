prepare() {
	echo -e "\nPreparing...\n"

	sudo mkdir /mnt/yantra
	export YDIR=/mnt/yantra

	cd ybuild

	chmod +x check.sh 01-toolchain.sh \
            02-system.sh 03-makeiso.sh

	./check.sh
	cd ..

	sudo rm /bin/sh
	sudo ln -s /bin/bash /bin/sh
}

install_dependencies() {
	echo -e "\nInstalling dependencies...\n"
	
	sudo apt update
	sudo apt-get install bison gawk gcc g++ make \
        texinfo grub-efi xorriso libssl-dev \
        pkg-config meson ninja-build \
        cargo rustc -y
}

install_yaps() {
	echo -e "\nInstalling yaps...\n"

	cd yaps
	[[ -d "yaps/build/" ]] || meson build --prefix /mnt/yantra/usr
	sudo ninja -C build install
	cd ..
}

make_iso() {
	echo -e "\nMaking iso...\n"

	cd ybuild
	./check.sh

	sudo ./01-toolchain.sh
	sudo ./02-system.sh
	sudo ./03-makeiso.sh

	cd ..
}

case $1 in

  "prepare")
    prepare
    ;;

  "install-dependencies")
    install_dependencies
    ;;
    
  "install-yaps")
    install_yaps
    ;;

  "make-iso")
    make_iso
    ;;

  *)
    [[ -z "$YDIR" ]] && prepare
    [[ -x "$(command -v cargo)" && -x "$(command -v gawk)" ]] || install_dependencies
    install_yaps
    make_iso
    ;;
esac
