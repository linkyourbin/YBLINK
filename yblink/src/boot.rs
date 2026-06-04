use core::mem::size_of;

core::arch::global_asm!(
    r#"
    .section .text.__pre_init, "ax"
    .align 2
    .global __pre_init
__pre_init:
    la t0, __sfast
    la t1, __efast
    la t2, __sifast
    bgeu t0, t1, 2f
1:
    lw t3, 0(t2)
    addi t2, t2, 4
    sw t3, 0(t0)
    addi t0, t0, 4
    bltu t0, t1, 1b
    fence.i
2:
    ret
    "#
);

const APP_LOAD_ADDR: u32 = 0x8000_3000;
const BOOT_HEADER_ADDR: u32 = 0x8000_1000;
const APP_OFFSET_FROM_BOOT_HEADER: u32 = APP_LOAD_ADDR - BOOT_HEADER_ADDR;

#[repr(C)]
struct FwInfoTable {
    offset: u32,
    size: u32,
    flags: u32,
    reserved0: u32,
    load_addr: u32,
    reserved1: u32,
    entry_point: u32,
    reserved2: u32,
    hash: [u8; 64],
    iv: [u8; 32],
}

#[repr(C)]
struct BootHeader {
    tag: u8,
    version: u8,
    length: u16,
    flags: u32,
    sw_version: u16,
    fuse_version: u8,
    fw_count: u8,
    dc_block_offset: u16,
    sig_block_offset: u16,
}

#[used]
#[unsafe(link_section = ".nor_cfg_option")]
static NOR_CFG_OPTION: [u32; 4] = [0xfcf9_0002, 0x0000_0005, 0x0000_1000, 0x0000_0000];

#[used]
#[unsafe(link_section = ".boot_header")]
static BOOT_HEADER: BootHeader = BootHeader {
    tag: 0xbf,
    version: 0x10,
    length: (size_of::<BootHeader>() + size_of::<FwInfoTable>()) as u16,
    flags: 0,
    sw_version: 0,
    fuse_version: 0,
    fw_count: 1,
    dc_block_offset: 0,
    sig_block_offset: 0,
};

#[used]
#[unsafe(link_section = ".fw_info_table")]
static FW_INFO: FwInfoTable = FwInfoTable {
    offset: APP_OFFSET_FROM_BOOT_HEADER,
    size: 0x0002_0000,
    flags: 0,
    reserved0: 0,
    load_addr: APP_LOAD_ADDR,
    reserved1: 0,
    entry_point: APP_LOAD_ADDR,
    reserved2: 0,
    hash: [0; 64],
    iv: [0; 32],
};
