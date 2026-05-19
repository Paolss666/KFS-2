#!/bin/sh
set -e

TOOLSDIR=${TOOLSDIR:-"$(cd `dirname $0` && pwd)"}
PROJECTROOT=${PROJECTROOT:-"${TOOLSDIR}/.."}
NAME=${NAME:-kfs}

cd "${PROJECTROOT}"

echo "[run.sh] Starting QEMU with VNC on port 5900"
echo "[run.sh] Connect with a VNC client to localhost:5900"

qemu-system-i386 \
    -cdrom "${NAME}.iso" \
    -vnc :0 \
    -no-reboot