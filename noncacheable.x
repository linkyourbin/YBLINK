SECTIONS
{
  .noncacheable (NOLOAD) : ALIGN(2048)
  {
    __noncacheable_start__ = .;
    KEEP(*(.noncacheable .noncacheable.*));
    KEEP(*(.noncacheable.bss .noncacheable.bss.*));
    . = ALIGN(8);
    __noncacheable_end__ = .;
  } > REGION_NONCACHEABLE_RAM
} INSERT AFTER .bss;
