#!/bin/sh

arm-none-eabi-objcopy -O binary $1 firmware.bin &&
  openocd -f board/ti_mspm0_launchpad.cfg -c \
    "init; halt;
    flash write_image erase firmware.bin 0x00000000 bin; 
    reset run"
