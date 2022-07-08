## Yantra System Steps

#### Using [isobuilder](https://github.com/yantraos/yaps-2.0/blob/main/isobuilder.sh)

```sh
chmod +x isobuilder.sh
./isobuilder.sh
```


#### Manual method

1. Take Host system as Ubuntu 18.04.5

```sh
git clone https://github.com/yantraos/yaps-2.0.git
mkdir /mnt/yantra
export YDIR=/mnt/yantra
cd yantra-1.0
cd ybuild
./check.sh
```

2. Install dependencies

```sh
sudo apt-get install bison gawk gcc g++ make \
    texinfo grub-efi xorriso libssl-dev \
    pkg-config meson ninja-build \
    cargo rustc -y
```

3. Replace sh with bash

```sh
sudo rm /bin/sh
sudo ln -s /bin/bash /bin/sh
```

4. Replace awk with gawk

```sh
sudo rm /usr/bin/awk 
sudo ln -s /usr/bin/gawk /usr/bin/awk
```

5. Install  yaps in hostsystem

```sh
cd yaps
sudo make install PREFIX=usr DESTDIR=/mnt/yantra/
```

6. Version check

```sh
cd ybuild
./check.sh
```

7. Run Toolchain

```sh
./01-toolchain.sh
```

8. Build System
```sh
./02-system.sh
```
9. Need to write another file for desktop pkgs
```sh
//./03-desktop.sh
```

10. Make iso
```sh
./03-makeiso.sh
```
