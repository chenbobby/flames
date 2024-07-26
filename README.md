# flames

A Linux profiler

## Developer Setup

Minimum Required Version for Linux: v5.8 (BPF ringbuf support)
Recommended OS: Ubuntu 22.04

1. Install `bpftool`

```
apt update
apt upgrade
apt install linux-tools-common
apt install linux-tools-generic
apt install linux-tools-$(uname -r)
bpftool -V
```

> The minimum required version for `bpftool` is v5.5. This is because the [relevant Linux patch](https://patchwork.ozlabs.org/project/netdev/patch/20191018103404.12999-1-jolsa@kernel.org/) landed in mainline Linux v5.5.

2. Install kernel headers for enabling `<linux/bpf.h>`

```
apt install linux-headers-$(uname -r)
ln -s /usr/include/x86_64-linux-gnu/asm /usr/include/asm

```

3. Install a Linux library for manipulating ELF files

```
apt install libelf-dev
```

4. 
