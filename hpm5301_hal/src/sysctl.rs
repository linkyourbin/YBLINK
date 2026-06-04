use crate::pac;

const RESOURCE_START: usize = 256;

const SYSCTL_PRESET1: u8 = 1 << 1;
const PLL0CLK0: u8 = 1;
const CPU0_DIV_360MHZ: u8 = 2;
const AHB_DIV_120MHZ: u8 = 3;
const DCDC_360MHZ_MV: u16 = 1175;
const PLL0_720MHZ_MFI: u8 = 30;
const PLL0_720MHZ_MFN: u32 = 0;
const PLL_DIV_1P0: u8 = 0;
const PLL_DIV_1P2: u8 = 1;
const PLL_DIV_1P8: u8 = 4;

pub const OSC24M_HZ: u32 = 24_000_000;

#[derive(Clone, Copy, Default)]
pub struct Config;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Resource {
    Ahb0 = 256,
    Lmm0 = 257,
    Mct0 = 258,
    Rom0 = 259,
    Can0 = 260,
    Can1 = 261,
    Can2 = 262,
    Can3 = 263,
    Ptpc = 264,
    Tmr0 = 269,
    Tmr1 = 270,
    Tmr2 = 271,
    Tmr3 = 272,
    I2c0 = 273,
    I2c1 = 274,
    I2c2 = 275,
    I2c3 = 276,
    Spi0 = 277,
    Spi1 = 278,
    Spi2 = 279,
    Spi3 = 280,
    Urt0 = 281,
    Urt1 = 282,
    Urt2 = 283,
    Urt3 = 284,
    Urt4 = 285,
    Urt5 = 286,
    Urt6 = 287,
    Urt7 = 288,
    Wdg0 = 289,
    Wdg1 = 290,
    Mbx0 = 291,
    Tsns = 292,
    Crc0 = 293,
    Adc0 = 294,
    Adc1 = 295,
    Dac0 = 296,
    Dac1 = 297,
    Acmp = 298,
    Opa0 = 299,
    Opa1 = 300,
    Mot0 = 301,
    Rng0 = 302,
    Sdp0 = 303,
    Kman = 304,
    Gpio = 305,
    Hdma = 306,
    Xpi0 = 307,
    Usb0 = 308,
    Ref0 = 309,
    Ref1 = 310,
}

pub fn init(_config: Config) {
    let sysctl = unsafe { pac::Sysctl::steal() };
    let pllctlv2 = unsafe { pac::Pllctlv2::steal() };

    init_xtal_clock_preset(&sysctl, &pllctlv2);

    enable_group0_resource(&sysctl, Resource::Ahb0 as usize);
    enable_group0_resource(&sysctl, Resource::Lmm0 as usize);
    enable_group0_resource(&sysctl, Resource::Mct0 as usize);
    enable_group0_resource(&sysctl, Resource::Rom0 as usize);
    enable_group0_resource(&sysctl, Resource::Tmr1 as usize);
    enable_group0_resource(&sysctl, Resource::Mot0 as usize);
    enable_group0_resource(&sysctl, Resource::Gpio as usize);
    enable_group0_resource(&sysctl, Resource::Hdma as usize);
    enable_group0_resource(&sysctl, Resource::Xpi0 as usize);
    configure_mchtmr_clock(&sysctl);
    configure_gptmr1_clock(&sysctl);

    unsafe {
        sysctl
            .affiliatecpu0()
            .set()
            .write_with_zero(|w| w.link().bits(1));
    }
}

pub fn init_clock_group() {
    init(Config);
}

pub fn enable_resource(resource: Resource) {
    let sysctl = unsafe { pac::Sysctl::steal() };
    enable_group0_resource(&sysctl, resource as usize);
}

fn enable_group0_resource(sysctl: &pac::Sysctl, resource: usize) {
    let index = (resource - RESOURCE_START) / 32;
    let offset = (resource - RESOURCE_START) % 32;
    let mask = 1u32 << offset;

    if sysctl.group0(index).value().read().link().bits() & mask != 0 {
        return;
    }

    unsafe {
        sysctl
            .group0(index)
            .set()
            .write_with_zero(|w| w.link().bits(mask));
    }

    while sysctl.resource(resource).read().loc_busy().bit_is_set() {
        core::hint::spin_loop();
    }
}

fn init_xtal_clock_preset(sysctl: &pac::Sysctl, pllctlv2: &pac::Pllctlv2) {
    if cpu0_uses_osc24m(sysctl) {
        unsafe {
            pllctlv2
                .xtal()
                .modify(|_, w| w.ramp_time().bits(32 * 1_000 * 9));
            sysctl
                .global00()
                .modify(|_, w| w.mux().bits(SYSCTL_PRESET1));
        }

        while pllctlv2.xtal().read().busy().bit_is_set() {
            core::hint::spin_loop();
        }
    }

    pllctlv2
        .pllpll0()
        .config()
        .modify(|_, w| w.refsel().clear_bit());
    pllctlv2
        .pllpll1()
        .config()
        .modify(|_, w| w.refsel().clear_bit());

    configure_cpu0_360mhz(sysctl, pllctlv2);
}

fn cpu0_uses_osc24m(sysctl: &pac::Sysctl) -> bool {
    let cpu0 = sysctl.clock_cpuclk_top_cpu0().read();
    cpu0.mux().bits() == 0 && cpu0.div().bits() == 0
}

fn configure_mchtmr_clock(sysctl: &pac::Sysctl) {
    unsafe {
        sysctl
            .clockclk_top_mct0()
            .write(|w| w.mux().bits(0).div().bits(0));
    }

    while sysctl.clockclk_top_mct0().read().loc_busy().bit_is_set() {
        core::hint::spin_loop();
    }
}

fn configure_gptmr1_clock(sysctl: &pac::Sysctl) {
    unsafe {
        sysctl
            .clockclk_top_tmr1()
            .write(|w| w.mux().bits(0).div().bits(0));
    }

    while sysctl.clockclk_top_tmr1().read().loc_busy().bit_is_set() {
        core::hint::spin_loop();
    }
}

fn configure_cpu0_360mhz(sysctl: &pac::Sysctl, pllctlv2: &pac::Pllctlv2) {
    set_dcdc_voltage(DCDC_360MHZ_MV);
    configure_cpu0_domain_clock(sysctl, PLL0CLK0, CPU0_DIV_360MHZ, AHB_DIV_120MHZ);

    set_pll0_postdiv(pllctlv2, 0, PLL_DIV_1P0);
    set_pll0_postdiv(pllctlv2, 1, PLL_DIV_1P2);
    set_pll0_postdiv(pllctlv2, 2, PLL_DIV_1P8);
    init_pll0_720mhz(pllctlv2);
}

fn set_dcdc_voltage(mv: u16) {
    let pcfg = unsafe { pac::Pcfg::steal() };

    unsafe {
        pcfg.dcdc_mode().modify(|_, w| w.volt().bits(mv));
    }

    while pcfg.dcdc_mode().read().ready().bit_is_clear() {
        core::hint::spin_loop();
    }
}

fn configure_cpu0_domain_clock(sysctl: &pac::Sysctl, source: u8, cpu_div: u8, ahb_sub_div: u8) {
    let origin_cpu_div = sysctl.clock_cpuclk_top_cpu0().read().div().bits() + 1;

    if origin_cpu_div == cpu_div {
        write_cpu0_clock(sysctl, source, cpu_div, ahb_sub_div - 1);
        wait_cpu0_clock(sysctl);
    }

    write_cpu0_clock(sysctl, source, cpu_div - 1, ahb_sub_div - 1);
    wait_cpu0_clock(sysctl);
}

fn write_cpu0_clock(sysctl: &pac::Sysctl, source: u8, div: u8, ahb_sub_div: u8) {
    unsafe {
        sysctl.clock_cpuclk_top_cpu0().write(|w| {
            w.mux()
                .bits(source)
                .div()
                .bits(div)
                .sub0_div()
                .bits(ahb_sub_div)
        });
    }
}

fn wait_cpu0_clock(sysctl: &pac::Sysctl) {
    while sysctl
        .clock_cpuclk_top_cpu0()
        .read()
        .glb_busy()
        .bit_is_set()
    {
        core::hint::spin_loop();
    }
}

fn set_pll0_postdiv(pllctlv2: &pac::Pllctlv2, clk: usize, div: u8) {
    unsafe {
        pllctlv2.pllpll0().div(clk).modify(|_, w| w.div().bits(div));
    }

    while !pll0_div_is_stable(pllctlv2, clk) {
        core::hint::spin_loop();
    }
}

fn init_pll0_720mhz(pllctlv2: &pac::Pllctlv2) {
    unsafe {
        pllctlv2
            .pllpll0()
            .mfn()
            .write(|w| w.mfn().bits(PLL0_720MHZ_MFN));
        pllctlv2
            .pllpll0()
            .mfi()
            .write(|w| w.mfi().bits(PLL0_720MHZ_MFI));
    }

    while !pll0_is_stable(pllctlv2) {
        core::hint::spin_loop();
    }
}

fn pll0_is_stable(pllctlv2: &pac::Pllctlv2) -> bool {
    let status = pllctlv2.pllpll0().mfi().read();
    status.enable().bit_is_clear()
        || (status.busy().bit_is_clear() && status.response().bit_is_set())
}

fn pll0_div_is_stable(pllctlv2: &pac::Pllctlv2, clk: usize) -> bool {
    let status = pllctlv2.pllpll0().div(clk).read();
    status.enable().bit_is_clear()
        || (status.busy().bit_is_clear() && status.response().bit_is_set())
}

pub(crate) fn configure_gptmr0_clock() {
    let sysctl = unsafe { pac::Sysctl::steal() };
    unsafe {
        sysctl
            .clockclk_top_tmr0()
            .write(|w| w.mux().bits(0).div().bits(0));
    }

    while sysctl.clockclk_top_tmr0().read().loc_busy().bit_is_set() {
        core::hint::spin_loop();
    }
}

pub(crate) fn configure_gptmr1_clock_for_driver() {
    let sysctl = unsafe { pac::Sysctl::steal() };
    configure_gptmr1_clock(&sysctl);
}
