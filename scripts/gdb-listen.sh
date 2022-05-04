#!/bin/bash
set -euo pipefail

exec openocd \
    -f interface/raspberrypi-swd.cfg \
    -f target/rp2040.cfg
