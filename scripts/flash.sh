#!/bin/bash
set -euo pipefail

exec openocd \
    -f interface/raspberrypi-swd.cfg \
    -f target/rp2040.cfg \
    -c "program $1 verify reset exit"
