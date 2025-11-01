## Baby HAL for REDACTED board 🤫

Not to be used under any circumstances

Build the ELF binary (ccs project): `gmake -k -j 16 all -O`
Convert to raw binary file: `arm-none-eabi-objcopy -O binary <elf binary> firmware.bin`
Flash the above-mentioned binary file:
`openocd -f board/ti_mspm0_launchpad.cfg -c "init; halt; flash write_image erase firmware.bin 0x00000000 bin; reset run; exit"`

## OpenOCD

Here OpenOCD must be built from source from the master branch as the support for <redacted> board is not there on the release version. Please install all the dependencies mentioned (yes each one is important, dont neglect).
Then do

```
./bootstrap
./configure
make
sudo make install
```

Now you have the correct version of openocd install
