PROVIDE(__nor_cfg_option_load_addr__ = ORIGIN(XPI0_HEADER) + 0x400);
PROVIDE(__boot_header_load_addr__ = ORIGIN(XPI0_HEADER) + 0x1000);
PROVIDE(__app_load_addr__ = ORIGIN(XPI0_APP));
PROVIDE(__app_offset__ = __app_load_addr__ - __boot_header_load_addr__);

SECTIONS
{
  .nor_cfg_option __nor_cfg_option_load_addr__ :
  {
    KEEP(*(.nor_cfg_option));
  } > XPI0_HEADER

  .boot_header __boot_header_load_addr__ :
  {
    __boot_header_start__ = .;
    KEEP(*(.boot_header));
    KEEP(*(.fw_info_table));
    __boot_header_end__ = .;
  } > XPI0_HEADER
} INSERT BEFORE .text.dummy;
