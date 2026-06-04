SECTIONS
{
  .fast : ALIGN(4)
  {
    __sfast = .;
    KEEP(*(.fast .fast.*));
    . = ALIGN(4);
    __efast = .;
  } > REGION_FASTTEXT AT > REGION_RODATA

  __sifast = LOADADDR(.fast);
} INSERT AFTER .init.rust;
