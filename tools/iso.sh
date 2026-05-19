#!/bin/sh
set -e

# Emplacements
TOOLSDIR=${TOOLSDIR:-"$(cd `dirname $0` && pwd)"}
PROJECTROOT=${PROJECTROOT:-"${TOOLSDIR}/.."}
NAME=${NAME:-kfs}
SYSROOT="${PROJECTROOT}/sysroot"
ISODIR="${SYSROOT}/iso"
TMP_GRUB=/tmp/core.img

cd "${PROJECTROOT}"

. "${TOOLSDIR}/build.sh"

cd "${PROJECTROOT}"

mkdir -p "${ISODIR}"
mkdir -p "${ISODIR}/boot"
mkdir -p "${ISODIR}/boot/grub"

cp "${SYSROOT}/boot/${NAME}" "${ISODIR}/boot/${NAME}"


cat > "${ISODIR}/boot/grub/grub.cfg" << EOF
menuentry "${NAME}" {
    multiboot /boot/${NAME}
    boot
}
EOF

grub-mkimage \
    --config="${ISODIR}/boot/grub/grub.cfg" \
    --output="${TMP_GRUB}" \
    --prefix="(cd)" \
    --format=i386-pc \
    multiboot \
    biosdisk \
    iso9660

dd if=/usr/lib/grub/i386-pc/cdboot.img \
   of="${ISODIR}/boot/grub/grub.img" \
   status=none

dd if="${TMP_GRUB}" \
   of="${ISODIR}/boot/grub/grub.img" \
   conv=notrunc \
   oflag=append \
   status=none

rm -f "${TMP_GRUB}"

xorriso \
    -out_charset utf-8 \
    -volid "KFS" \
    -publisher "42" \
    -pathspecs on \
    -outdev "${PROJECTROOT}/${NAME}.iso" \
    -blank as_needed \
    -boot_image grub bin_path=/boot/grub/grub.img \
    -boot_image grub cat_path=/boot.catalog \
    -boot_image grub emul_type=no_emulation \
    -boot_image grub boot_info_table=on \
    -boot_image grub load_size=4096 \
    -map "${ISODIR}" /

echo "[iso.sh] ISO built successfully at ${PROJECTROOT}/${NAME}.iso"
ls -lh "${PROJECTROOT}/${NAME}.iso"