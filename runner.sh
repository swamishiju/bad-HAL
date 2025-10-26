#!/bin/sh

OBJCOPY=arm-none-eabi-objcopy


if ! command -v $OBJCOPY >/dev/null 2>&1
then
    OBJCOPY=~/ti/gcc_arm_none_eabi_9_2_1/arm-none-eabi/bin/objcopy
    if ! command -v $OBJCOPY >/dev/null 2>&1
    then
        echo "OBJCOPY not found!!"
        exit 1
    fi
fi

$OBJCOPY -O binary $1 firmware.bin &&
openocd -f board/ti_mspm0_launchpad.cfg -c \
    "init; halt;
    flash write_image erase firmware.bin 0x00000000 bin; 
    reset run; exit"
