/* memory.x — MSPM0L2228 LaunchPad */

MEMORY
{
  FLASH (rx)      : ORIGIN = 0x00000000, LENGTH = 0x00040000  /* 256 KB */
  SRAM  (rwx)     : ORIGIN = 0x20200000, LENGTH = 0x00008000  /* 32 KB */
  BCR_CONFIG (r)  : ORIGIN = 0x41C00000, LENGTH = 0x000000FF
  BSL_CONFIG (r)  : ORIGIN = 0x41C00100, LENGTH = 0x00000080
}

/* Define initial stack pointer — top of SRAM */
_estack = ORIGIN(SRAM) + LENGTH(SRAM);

SECTIONS
{
  .intvecs :
  {
    LONG(_estack)
    KEEP(*(.intvecs))
  } > FLASH

  .text :
  {
    *(.text*)
    *(.rodata*)
    *(.const*)
  } > FLASH
	

  .data : AT (ADDR(.text) + SIZEOF(.text))
  {
    __sdata = .;
    *(.data*)
    __edata = .;
  } > SRAM

  .bss :
  {
    __sbss = .;
    *(.bss*)
    *(COMMON)
    __ebss = .;
  } > SRAM

  .stack (NOLOAD) :
  {
    . = ALIGN(8);
    _sstack = .;
    . += 4K; /* optional: reserve 4 KB for stack */
    _estack_marker = .;
  } > SRAM

  /* Optional config sections */
  .BCRConfig : { KEEP(*(.BCRConfig*)) } > BCR_CONFIG
  .BSLConfig : { KEEP(*(.BSLConfig*)) } > BSL_CONFIG
  

  /DISCARD/ : { 
  *(.eh_frame*)
  *(.stab*)
  *(.debug*)
   }
}
