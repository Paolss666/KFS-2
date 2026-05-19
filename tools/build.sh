#!/bin/sh
set -e

TOOLSDIR=${TOOLSDIR:-"$(cd `dirname $0` && pwd)"}
PROJECTROOT=${PROJECTROOT:-"${TOOLSDIR}/.."}
NAME=${NAME:-kfs}
BUILDDIR="${PROJECTROOT}/build"
SYSROOT="${PROJECTROOT}/sysroot"
ISODIR="${SYSROOT}/iso"

cd "${PROJECTROOT}"

mkdir -p "${BUILDDIR}"
mkdir -p "${SYSROOT}/boot"

cargo build \
    --target i386-kfs.json \
    -Zbuild-std=core \
    -Zjson-target-spec \
    --release

nasm -f elf32 asm/boot.asm -o "${BUILDDIR}/boot.o"

ld -m elf_i386 -T linker.ld \
    -o "${SYSROOT}/boot/${NAME}" \
    "${BUILDDIR}/boot.o" \
    target/i386-kfs/release/libkfs_1.a

echo "[build.sh] Kernel built successfully at ${SYSROOT}/boot/${NAME}"