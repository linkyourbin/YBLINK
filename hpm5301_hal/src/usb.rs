use core::cell::UnsafeCell;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};
use core::task::Poll;

use embassy_usb_driver::{
    Bus as UsbBus, ControlPipe as UsbControlPipe, Direction, Driver, EndpointAddress,
    EndpointAllocError, EndpointError, EndpointIn, EndpointInfo, EndpointOut, EndpointType, Event,
    Unsupported,
};

use crate::{Peri, pac, peripherals, sysctl, time};

const USB_BASE: usize = 0xf300_c000;

const REG_DEVICEADDR: usize = 0x154;
const REG_ENDPTLISTADDR: usize = 0x158;
const REG_USBCMD: usize = 0x140;
const REG_USBSTS: usize = 0x144;
const REG_USBINTR: usize = 0x148;
const REG_PORTSC1: usize = 0x184;
const REG_OTGSC: usize = 0x1a4;
const REG_USBMODE: usize = 0x1a8;
const REG_ENDPTSETUPSTAT: usize = 0x1ac;
const REG_ENDPTPRIME: usize = 0x1b0;
const REG_ENDPTFLUSH: usize = 0x1b4;
const REG_ENDPTSTAT: usize = 0x1b8;
const REG_ENDPTCOMPLETE: usize = 0x1bc;
const REG_ENDPTCTRL: usize = 0x1c0;
const REG_ENDPTNAK: usize = 0x178;
const REG_ENDPTNAKEN: usize = 0x17c;
const REG_OTG_CTRL0: usize = 0x200;
const REG_PHY_CTRL0: usize = 0x210;
const REG_PHY_CTRL1: usize = 0x214;
const REG_PHY_STATUS: usize = 0x224;

const USBCMD_RS: u32 = 1 << 0;
const USBCMD_RST: u32 = 1 << 1;
const USBCMD_ITC_MASK: u32 = 0xff << 16;

const USBSTS_UI: u32 = 1 << 0;
const USBSTS_PCI: u32 = 1 << 2;
const USBSTS_URI: u32 = 1 << 6;
const USBSTS_SLI: u32 = 1 << 8;

const USBINTR_UE: u32 = 1 << 0;
const USBINTR_UEE: u32 = 1 << 1;
const USBINTR_PCE: u32 = 1 << 2;
const USBINTR_URE: u32 = 1 << 6;
const USBINTR_SLE: u32 = 1 << 8;

const USBMODE_CM_DEVICE: u32 = 0b10;
const USBMODE_ES: u32 = 1 << 2;
const USBMODE_SLOM: u32 = 1 << 3;

const PORTSC1_CCS: u32 = 1 << 0;
const PORTSC1_FPR: u32 = 1 << 6;
const PORTSC1_SUSP: u32 = 1 << 7;
const PORTSC1_PFSC: u32 = 1 << 24;
const PORTSC1_PTW: u32 = 1 << 28;
const PORTSC1_STS: u32 = 1 << 29;

const OTGSC_VD: u32 = 1 << 0;
const OTGSC_ASV: u32 = 1 << 10;
const OTGSC_ASVIS: u32 = 1 << 18;
const OTGSC_ASVIE: u32 = 1 << 26;

const OTG_CTRL0_POWER_MASK: u32 = 1 << 9;
const OTG_CTRL0_UTMI_RESET_SW: u32 = 1 << 11;
const OTG_CTRL0_UTMI_SUSPENDM_SW: u32 = 1 << 12;
const OTG_CTRL0_WKDPDMCHG_EN: u32 = 1 << 25;

const PHY_CTRL0_DP_DM_PULLDOWN_BITS: u32 = 0x0010_00e0;
const PHY_CTRL0_VBUS_VALID_OVERRIDE_EN: u32 = 1 << 0;
const PHY_CTRL0_SESS_VALID_OVERRIDE_EN: u32 = 1 << 1;
const PHY_CTRL0_VBUS_VALID_OVERRIDE: u32 = 1 << 12;
const PHY_CTRL0_SESS_VALID_OVERRIDE: u32 = 1 << 13;

const PHY_CTRL1_UTMI_OTG_SUSPENDM: u32 = 1 << 1;
const PHY_CTRL1_UTMI_CFG_RST_N: u32 = 1 << 20;
const PHY_STATUS_UTMI_CLK_VALID: u32 = 1 << 31;

const ENDPTCTRL_RXS: u32 = 1 << 0;
const ENDPTCTRL_RXT_MASK: u32 = 0b11 << 2;
const ENDPTCTRL_RXR: u32 = 1 << 6;
const ENDPTCTRL_RXE: u32 = 1 << 7;
const ENDPTCTRL_TXS: u32 = 1 << 16;
const ENDPTCTRL_TXT_MASK: u32 = 0b11 << 18;
const ENDPTCTRL_TXR: u32 = 1 << 22;
const ENDPTCTRL_TXE: u32 = 1 << 23;

const ENDPOINT_COUNT: usize = 16;
const QTD_COUNT_EACH_QHD: usize = 8;
const QHD_ITEM_SIZE: usize = 64;
const QTD_ITEM_SIZE: usize = 32;
const QHD_LIST_SIZE: usize = QHD_ITEM_SIZE * ENDPOINT_COUNT * 2;
const QTD_LIST_SIZE: usize = QTD_ITEM_SIZE * ENDPOINT_COUNT * 2 * QTD_COUNT_EACH_QHD;

#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct Config {
    pub force_full_speed: bool,
}

#[repr(C, align(2048))]
struct QhdListData(UnsafeCell<[u8; QHD_LIST_SIZE]>);

impl QhdListData {
    const fn new() -> Self {
        Self(UnsafeCell::new([0; QHD_LIST_SIZE]))
    }
}

#[repr(C, align(32))]
struct QtdListData(UnsafeCell<[u8; QTD_LIST_SIZE]>);

impl QtdListData {
    const fn new() -> Self {
        Self(UnsafeCell::new([0; QTD_LIST_SIZE]))
    }
}

#[repr(C)]
pub struct EndpointState {
    qhd_list: QhdListData,
    qtd_list: QtdListData,
    alloc_mask: AtomicU32,
}

unsafe impl Sync for EndpointState {}

impl EndpointState {
    pub const fn new() -> Self {
        Self {
            qhd_list: QhdListData::new(),
            qtd_list: QtdListData::new(),
            alloc_mask: AtomicU32::new(0),
        }
    }

    fn try_acquire(&self) -> bool {
        const STATE_IN_USE: u32 = 1 << 31;
        self.alloc_mask.fetch_or(STATE_IN_USE, Ordering::SeqCst) & STATE_IN_USE == 0
    }

    fn qhd_list_addr(&self) -> u32 {
        self.qhd_list.0.get() as u32
    }

    unsafe fn qhd_word(&self, qhd: usize, word: usize) -> *mut u32 {
        unsafe {
            self.qhd_list
                .0
                .get()
                .cast::<u8>()
                .add(qhd * QHD_ITEM_SIZE + word * 4)
                .cast()
        }
    }

    unsafe fn qtd_word(&self, qtd: usize, word: usize) -> *mut u32 {
        unsafe {
            self.qtd_list
                .0
                .get()
                .cast::<u8>()
                .add(qtd * QTD_ITEM_SIZE + word * 4)
                .cast()
        }
    }

    unsafe fn qhd_read(&self, qhd: usize, word: usize) -> u32 {
        unsafe { self.qhd_word(qhd, word).read_volatile() }
    }

    unsafe fn qhd_write(&self, qhd: usize, word: usize, value: u32) {
        unsafe { self.qhd_word(qhd, word).write_volatile(value) };
    }

    unsafe fn qhd_modify(&self, qhd: usize, word: usize, f: impl FnOnce(u32) -> u32) {
        let value = unsafe { self.qhd_read(qhd, word) };
        unsafe { self.qhd_write(qhd, word, f(value)) };
    }

    unsafe fn qtd_read(&self, qtd: usize, word: usize) -> u32 {
        unsafe { self.qtd_word(qtd, word).read_volatile() }
    }

    unsafe fn qtd_write(&self, qtd: usize, word: usize, value: u32) {
        unsafe { self.qtd_word(qtd, word).write_volatile(value) };
    }

    unsafe fn qtd_modify(&self, qtd: usize, word: usize, f: impl FnOnce(u32) -> u32) {
        let value = unsafe { self.qtd_read(qtd, word) };
        unsafe { self.qtd_write(qtd, word, f(value)) };
    }

    unsafe fn qhd_reset(&self, qhd: usize) {
        for word in 0..(QHD_ITEM_SIZE / 4) {
            unsafe { self.qhd_write(qhd, word, 0) };
        }
    }

    unsafe fn qtd_reset(&self, qtd: usize) {
        for word in 0..(QTD_ITEM_SIZE / 4) {
            unsafe { self.qtd_write(qtd, word, 0) };
        }
    }

    unsafe fn setup_request(&self) -> [u8; 8] {
        let lo = unsafe { self.qhd_read(0, 10) }.to_le_bytes();
        let hi = unsafe { self.qhd_read(0, 11) }.to_le_bytes();
        [lo[0], lo[1], lo[2], lo[3], hi[0], hi[1], hi[2], hi[3]]
    }

    unsafe fn reset_dcd_data(&self, ep0_max_packet_size: u16) {
        for i in 0..ENDPOINT_COUNT * 2 {
            unsafe { self.qhd_reset(i) };
        }
        for i in 0..ENDPOINT_COUNT * 2 * QTD_COUNT_EACH_QHD {
            unsafe { self.qtd_reset(i) };
        }

        unsafe {
            self.qhd_set_cap(0, ep0_max_packet_size, EndpointType::Control, true);
            self.qhd_set_cap(1, ep0_max_packet_size, EndpointType::Control, false);
            self.qhd_write(0, 2, 1);
            self.qhd_write(1, 2, 1);
        }
    }

    unsafe fn init_qhd(&self, ep_config: &EpConfig) {
        let ep_num = ep_config.ep_addr.index();
        let ep_idx = 2 * ep_num + if ep_config.ep_addr.is_in() { 1 } else { 0 };

        unsafe {
            self.qhd_reset(ep_idx);
            self.qhd_set_cap(
                ep_idx,
                ep_config.max_packet_size,
                ep_config.transfer,
                ep_config.transfer == EndpointType::Control,
            );
            self.qhd_write(ep_idx, 2, 1);
        }
    }

    unsafe fn qhd_set_cap(
        &self,
        qhd: usize,
        max_packet_size: u16,
        transfer: EndpointType,
        ios: bool,
    ) {
        let mut cap = (u32::from(max_packet_size & 0x07ff)) << 16;
        cap |= 1 << 29;
        if ios {
            cap |= 1 << 15;
        }
        if transfer == EndpointType::Isochronous {
            let mult = (((max_packet_size >> 11) & 0x03) as u32 + 1) & 0x03;
            cap |= mult << 30;
        }
        unsafe { self.qhd_write(qhd, 0, cap) };
    }

    unsafe fn qtd_reinit_with(&self, qtd: usize, data: &[u8], transfer_bytes: usize) {
        unsafe { self.qtd_reset(qtd) };
        unsafe {
            self.qtd_write(qtd, 1, ((transfer_bytes as u32) << 16) | (1 << 7));
            self.qtd_write(qtd, 7, transfer_bytes as u32);
        }

        if transfer_bytes < 0x4000 {
            unsafe { self.qtd_write(qtd, 0, 1) };
        }

        if transfer_bytes == 0 {
            unsafe { self.qtd_write(qtd, 2, 0) };
            return;
        }

        let sys_addr = local_to_sys_address(data.as_ptr() as u32);
        unsafe {
            self.qtd_write(qtd, 2, sys_addr);
            for i in 1..5 {
                self.qtd_write(
                    qtd,
                    2 + i,
                    (sys_addr & 0xffff_f000).wrapping_add((i as u32) << 12),
                );
            }
        }
    }

    unsafe fn qhd_total_remaining(&self, ep_idx: usize) -> usize {
        ((unsafe { self.qhd_read(ep_idx, 3) } >> 16) & 0x7fff) as usize
    }

    unsafe fn qtd_addr(&self, qtd: usize) -> u32 {
        unsafe { self.qtd_word(qtd, 0) as u32 }
    }
}

impl Default for EndpointState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
struct EndpointAllocData {
    info: EndpointInfo,
    used: bool,
}

impl EndpointAllocData {
    fn new(dir: Direction) -> Self {
        Self {
            info: EndpointInfo {
                addr: EndpointAddress::from_parts(0, dir),
                max_packet_size: 0,
                ep_type: EndpointType::Bulk,
                interval_ms: 0,
            },
            used: false,
        }
    }
}

pub struct UsbDriver<'d> {
    endpoints_in: [EndpointAllocData; ENDPOINT_COUNT],
    endpoints_out: [EndpointAllocData; ENDPOINT_COUNT],
    config: Config,
    ep_state: &'d EndpointState,
}

impl<'d> UsbDriver<'d> {
    pub fn new(
        _peri: Peri<'d, peripherals::USB0>,
        _dm: Peri<'d, peripherals::PA24>,
        _dp: Peri<'d, peripherals::PA25>,
        config: Config,
        ep_state: &'d EndpointState,
    ) -> Self {
        assert!(
            ep_state.try_acquire(),
            "EndpointState is already in use by another USB driver"
        );

        sysctl::enable_resource(sysctl::Resource::Usb0);
        configure_usb_pins();

        reg_modify(REG_PHY_CTRL0, |v| v | PHY_CTRL0_DP_DM_PULLDOWN_BITS);
        reg_modify(REG_OTG_CTRL0, |v| v | OTG_CTRL0_POWER_MASK);
        time::delay_millis(100);
        reg_modify(REG_PHY_CTRL0, |v| {
            v | PHY_CTRL0_VBUS_VALID_OVERRIDE
                | PHY_CTRL0_SESS_VALID_OVERRIDE
                | PHY_CTRL0_VBUS_VALID_OVERRIDE_EN
                | PHY_CTRL0_SESS_VALID_OVERRIDE_EN
        });

        Self {
            endpoints_in: core::array::from_fn(|_| EndpointAllocData::new(Direction::In)),
            endpoints_out: core::array::from_fn(|_| EndpointAllocData::new(Direction::Out)),
            config,
            ep_state,
        }
    }

    fn find_free_endpoint(&mut self, ep_type: EndpointType, dir: Direction) -> Option<usize> {
        let endpoint_list = match dir {
            Direction::Out => &mut self.endpoints_out,
            Direction::In => &mut self.endpoints_in,
        };
        endpoint_list
            .iter()
            .enumerate()
            .find(|(i, ep)| (*i != 0 || ep_type == EndpointType::Control) && !ep.used)
            .map(|(i, _)| i)
    }
}

impl<'d> Driver<'d> for UsbDriver<'d> {
    type EndpointOut = Endpoint<'d, Out>;
    type EndpointIn = Endpoint<'d, In>;
    type ControlPipe = ControlPipe<'d>;
    type Bus = Bus<'d>;

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, EndpointAllocError> {
        let ep_idx = if let Some(addr) = ep_addr {
            let idx = addr.index();
            if idx >= self.endpoints_out.len() || self.endpoints_out[idx].used {
                return Err(EndpointAllocError);
            }
            idx
        } else {
            self.find_free_endpoint(ep_type, Direction::Out)
                .ok_or(EndpointAllocError)?
        };

        let ep = EndpointInfo {
            addr: EndpointAddress::from_parts(ep_idx, Direction::Out),
            ep_type,
            max_packet_size,
            interval_ms,
        };
        self.endpoints_out[ep_idx].used = true;
        self.endpoints_out[ep_idx].info = ep;
        Ok(Endpoint {
            _phantom: PhantomData,
            ep_state: self.ep_state,
            info: ep,
        })
    }

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, EndpointAllocError> {
        let ep_idx = if let Some(addr) = ep_addr {
            let idx = addr.index();
            if idx >= self.endpoints_in.len() || self.endpoints_in[idx].used {
                return Err(EndpointAllocError);
            }
            idx
        } else {
            self.find_free_endpoint(ep_type, Direction::In)
                .ok_or(EndpointAllocError)?
        };

        let ep = EndpointInfo {
            addr: EndpointAddress::from_parts(ep_idx, Direction::In),
            ep_type,
            max_packet_size,
            interval_ms,
        };
        self.endpoints_in[ep_idx].used = true;
        self.endpoints_in[ep_idx].info = ep;
        Ok(Endpoint {
            _phantom: PhantomData,
            ep_state: self.ep_state,
            info: ep,
        })
    }

    fn start(mut self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        let ep_out = self
            .alloc_endpoint_out(EndpointType::Control, None, control_max_packet_size, 0)
            .unwrap();
        let ep_in = self
            .alloc_endpoint_in(EndpointType::Control, None, control_max_packet_size, 0)
            .unwrap();

        let mut endpoints_in: [EndpointInfo; ENDPOINT_COUNT] =
            core::array::from_fn(|_| EndpointInfo {
                addr: EndpointAddress::from_parts(0, Direction::In),
                ep_type: EndpointType::Bulk,
                max_packet_size: 0,
                interval_ms: 0,
            });
        let mut endpoints_out: [EndpointInfo; ENDPOINT_COUNT] =
            core::array::from_fn(|_| EndpointInfo {
                addr: EndpointAddress::from_parts(0, Direction::Out),
                ep_type: EndpointType::Bulk,
                max_packet_size: 0,
                interval_ms: 0,
            });

        endpoints_in[0] = ep_in.info;
        endpoints_out[0] = ep_out.info;
        for i in 1..ENDPOINT_COUNT {
            endpoints_in[i] = self.endpoints_in[i].info;
            endpoints_out[i] = self.endpoints_out[i].info;
        }

        (
            Bus {
                endpoints_in,
                endpoints_out,
                inited: false,
                config: self.config,
                ep_state: self.ep_state,
            },
            ControlPipe {
                max_packet_size: control_max_packet_size as usize,
                ep_in,
                ep_out,
            },
        )
    }
}

pub struct Bus<'d> {
    endpoints_in: [EndpointInfo; ENDPOINT_COUNT],
    endpoints_out: [EndpointInfo; ENDPOINT_COUNT],
    inited: bool,
    config: Config,
    ep_state: &'d EndpointState,
}

impl UsbBus for Bus<'_> {
    async fn enable(&mut self) {
        self.device_init();
        reg_write(
            REG_ENDPTLISTADDR,
            local_to_sys_address(self.ep_state.qhd_list_addr()),
        );
        reg_write(REG_USBSTS, reg_read(REG_USBSTS));
        reg_write(
            REG_USBINTR,
            USBINTR_UE | USBINTR_UEE | USBINTR_PCE | USBINTR_URE | USBINTR_SLE,
        );
        reg_modify(REG_OTGSC, |v| (v | OTGSC_ASVIS | OTGSC_ASVIE) & !OTGSC_VD);
        self.device_connect();
    }

    async fn disable(&mut self) {
        self.device_deinit();
    }

    async fn poll(&mut self) -> Event {
        poll_fn(|cx| {
            let status = reg_read(REG_USBSTS);
            debug_mark(1, status);

            if !self.inited {
                self.inited = true;
                if reg_read(REG_OTGSC) & OTGSC_ASV != 0 {
                    return Poll::Ready(Event::PowerDetected);
                }
                cx.waker().wake_by_ref();
                return Poll::Ready(Event::PowerRemoved);
            }

            if reg_read(REG_ENDPTSETUPSTAT) & 1 != 0 {
                debug_mark(1, 0x5301_1101);
                return Poll::Pending;
            }

            if reg_read(REG_OTGSC) & OTGSC_ASVIS != 0 {
                reg_modify(REG_OTGSC, |v| v | OTGSC_ASVIS);
                if reg_read(REG_OTGSC) & OTGSC_ASV != 0 {
                    return Poll::Ready(Event::PowerDetected);
                }
                return Poll::Ready(Event::PowerRemoved);
            }

            if status & USBSTS_URI != 0 {
                debug_mark(1, 0x5301_1140);
                reg_write(REG_USBSTS, USBSTS_URI);

                for i in 1..ENDPOINT_COUNT {
                    self.endpoint_close(EndpointAddress::from_parts(i, Direction::In));
                    self.endpoint_close(EndpointAddress::from_parts(i, Direction::Out));
                }
                debug_mark(1, 0x5301_1141);

                self.device_set_address(0);
                self.endpoint_open(EpConfig {
                    transfer: EndpointType::Control,
                    ep_addr: EndpointAddress::from_parts(0, Direction::In),
                    max_packet_size: 64,
                });
                self.endpoint_open(EpConfig {
                    transfer: EndpointType::Control,
                    ep_addr: EndpointAddress::from_parts(0, Direction::Out),
                    max_packet_size: 64,
                });
                self.device_bus_reset(64);
                reg_write(REG_USBSTS, reg_read(REG_USBSTS));
                reg_modify(REG_USBINTR, |v| v | USBINTR_UE);
                debug_mark(1, 0x5301_1142);
                return Poll::Ready(Event::Reset);
            }

            if status & USBSTS_SLI != 0 {
                reg_write(REG_USBSTS, USBSTS_SLI);
                if reg_read(REG_PORTSC1) & PORTSC1_SUSP != 0 {
                    return Poll::Ready(Event::Suspend);
                }
            }

            if status & USBSTS_PCI != 0 {
                reg_write(REG_USBSTS, USBSTS_PCI);
                if reg_read(REG_PORTSC1) & PORTSC1_CCS != 0 {
                    return Poll::Ready(Event::Resume);
                }
            }

            cx.waker().wake_by_ref();
            Poll::Pending
        })
        .await
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        if enabled {
            let endpoint_list = if ep_addr.direction() == Direction::In {
                self.endpoints_in
            } else {
                self.endpoints_out
            };
            let ep_data = endpoint_list[ep_addr.index()];
            self.endpoint_open(EpConfig {
                transfer: ep_data.ep_type,
                ep_addr,
                max_packet_size: ep_data.max_packet_size,
            });
        } else {
            self.endpoint_close(ep_addr);
        }
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        let reg = REG_ENDPTCTRL + ep_addr.index() * 4;
        if stalled {
            if ep_addr.is_in() {
                reg_modify(reg, |v| v | ENDPTCTRL_TXS);
            } else {
                reg_modify(reg, |v| v | ENDPTCTRL_RXS);
            }
        } else if ep_addr.is_in() {
            reg_modify(reg, |v| (v | ENDPTCTRL_TXR) & !ENDPTCTRL_TXS);
        } else {
            reg_modify(reg, |v| (v | ENDPTCTRL_RXR) & !ENDPTCTRL_RXS);
        }
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        let value = reg_read(REG_ENDPTCTRL + ep_addr.index() * 4);
        if ep_addr.is_in() {
            value & ENDPTCTRL_TXS != 0
        } else {
            value & ENDPTCTRL_RXS != 0
        }
    }

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        reg_modify(REG_PORTSC1, |v| v | PORTSC1_FPR);
        while reg_read(REG_PORTSC1) & PORTSC1_FPR != 0 {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

impl Bus<'_> {
    fn phy_init(&mut self) {
        reg_modify(REG_PHY_CTRL0, |v| v & !PHY_CTRL0_DP_DM_PULLDOWN_BITS);
        reg_modify(REG_OTG_CTRL0, |v| {
            (v | OTG_CTRL0_UTMI_RESET_SW) & !OTG_CTRL0_UTMI_SUSPENDM_SW
        });
        reg_modify(REG_PHY_CTRL1, |v| v & !PHY_CTRL1_UTMI_CFG_RST_N);

        while reg_read(REG_OTG_CTRL0) & OTG_CTRL0_UTMI_RESET_SW == 0 {
            core::hint::spin_loop();
        }

        reg_modify(REG_OTG_CTRL0, |v| v | OTG_CTRL0_UTMI_SUSPENDM_SW);
        time::delay_micros(5);
        reg_modify(REG_OTG_CTRL0, |v| {
            (v & !OTG_CTRL0_WKDPDMCHG_EN) & !OTG_CTRL0_UTMI_RESET_SW
        });
        reg_modify(REG_PHY_STATUS, |v| v | PHY_STATUS_UTMI_CLK_VALID);
        while reg_read(REG_PHY_STATUS) & PHY_STATUS_UTMI_CLK_VALID == 0 {
            core::hint::spin_loop();
        }
        reg_modify(REG_PHY_CTRL1, |v| {
            v | PHY_CTRL1_UTMI_CFG_RST_N | PHY_CTRL1_UTMI_OTG_SUSPENDM
        });
    }

    fn phy_deinit(&mut self) {
        reg_modify(REG_OTG_CTRL0, |v| {
            (v | OTG_CTRL0_UTMI_SUSPENDM_SW) & !OTG_CTRL0_UTMI_RESET_SW
        });
        reg_modify(REG_PHY_CTRL1, |v| {
            v & !(PHY_CTRL1_UTMI_CFG_RST_N | PHY_CTRL1_UTMI_OTG_SUSPENDM)
        });
    }

    fn device_init(&mut self) {
        self.phy_init();
        reg_modify(REG_USBCMD, |v| v | USBCMD_RST);
        while reg_read(REG_USBCMD) & USBCMD_RST != 0 {
            core::hint::spin_loop();
        }

        reg_modify(REG_USBMODE, |v| (v & !0b11) | USBMODE_CM_DEVICE);
        reg_modify(REG_USBMODE, |v| {
            (v & !(USBMODE_ES | USBMODE_SLOM)) | USBMODE_CM_DEVICE
        });

        reg_modify(REG_PORTSC1, |v| {
            let mut value = v & !(PORTSC1_STS | PORTSC1_PTW | PORTSC1_PFSC);
            if self.config.force_full_speed {
                value |= PORTSC1_PFSC;
            }
            value
        });

        reg_modify(REG_USBCMD, |v| v & !USBCMD_ITC_MASK);
        reg_modify(REG_OTGSC, |v| v | OTGSC_VD);
    }

    fn device_deinit(&mut self) {
        reg_modify(REG_USBCMD, |v| v & !USBCMD_RS);
        reg_modify(REG_USBCMD, |v| v | USBCMD_RST);
        while reg_read(REG_USBCMD) & USBCMD_RST != 0 {
            core::hint::spin_loop();
        }
        self.phy_deinit();
        reg_write(REG_ENDPTLISTADDR, 0);
        reg_write(REG_USBSTS, 0xffff_ffff);
        reg_write(REG_USBINTR, 0);
    }

    fn device_connect(&mut self) {
        reg_modify(REG_USBCMD, |v| v | USBCMD_RS);
    }

    fn device_set_address(&mut self, addr: u8) {
        reg_write(REG_DEVICEADDR, (u32::from(addr & 0x7f) << 25) | (1 << 24));
    }

    fn device_bus_reset(&mut self, ep0_max_packet_size: u16) {
        for i in 1..ENDPOINT_COUNT {
            reg_write(
                REG_ENDPTCTRL + i * 4,
                ((EndpointType::Bulk as u32) << 18) | ((EndpointType::Bulk as u32) << 2),
            );
        }

        reg_write(REG_ENDPTNAK, reg_read(REG_ENDPTNAK));
        reg_write(REG_ENDPTNAKEN, 0);
        reg_write(REG_USBSTS, reg_read(REG_USBSTS));
        reg_write(REG_ENDPTSETUPSTAT, reg_read(REG_ENDPTSETUPSTAT));
        reg_write(REG_ENDPTCOMPLETE, reg_read(REG_ENDPTCOMPLETE));

        while reg_read(REG_ENDPTPRIME) != 0 {
            core::hint::spin_loop();
        }
        reg_write(REG_ENDPTFLUSH, 0xffff_ffff);
        while reg_read(REG_ENDPTFLUSH) != 0 {
            core::hint::spin_loop();
        }

        unsafe { self.ep_state.reset_dcd_data(ep0_max_packet_size) };
    }

    fn endpoint_open(&mut self, ep_config: EpConfig) {
        if ep_config.ep_addr.index() >= ENDPOINT_COUNT {
            return;
        }

        unsafe { self.ep_state.init_qhd(&ep_config) };

        let ep_num = ep_config.ep_addr.index();
        let reg = REG_ENDPTCTRL + ep_num * 4;
        let transfer = ep_config.transfer as u32;

        if ep_config.ep_addr.is_in() {
            reg_modify(reg, |v| {
                (v & !ENDPTCTRL_TXT_MASK) | ENDPTCTRL_TXE | ENDPTCTRL_TXR | (transfer << 18)
            });
        } else {
            reg_modify(reg, |v| {
                (v & !ENDPTCTRL_RXT_MASK) | ENDPTCTRL_RXE | ENDPTCTRL_RXR | (transfer << 2)
            });
        }
    }

    fn endpoint_close(&mut self, ep_addr: EndpointAddress) {
        if ep_addr.index() >= ENDPOINT_COUNT {
            return;
        }

        let ep_bit = 1u32 << ep_addr.index();
        if ep_addr.is_in() {
            loop {
                reg_write(REG_ENDPTFLUSH, ep_bit << 16);
                while reg_read(REG_ENDPTFLUSH) & (ep_bit << 16) != 0 {
                    core::hint::spin_loop();
                }
                if reg_read(REG_ENDPTSTAT) & (ep_bit << 16) == 0 {
                    break;
                }
            }
            reg_modify(REG_ENDPTCTRL + ep_addr.index() * 4, |v| {
                (v & !(ENDPTCTRL_TXE | ENDPTCTRL_TXS | ENDPTCTRL_TXT_MASK))
                    | ((EndpointType::Bulk as u32) << 18)
            });
        } else {
            loop {
                reg_write(REG_ENDPTFLUSH, ep_bit);
                while reg_read(REG_ENDPTFLUSH) & ep_bit != 0 {
                    core::hint::spin_loop();
                }
                if reg_read(REG_ENDPTSTAT) & ep_bit == 0 {
                    break;
                }
            }
            reg_modify(REG_ENDPTCTRL + ep_addr.index() * 4, |v| {
                (v & !(ENDPTCTRL_RXE | ENDPTCTRL_RXS | ENDPTCTRL_RXT_MASK))
                    | ((EndpointType::Bulk as u32) << 2)
            });
        }
    }
}

struct EpConfig {
    transfer: EndpointType,
    ep_addr: EndpointAddress,
    max_packet_size: u16,
}

pub enum In {}
pub enum Out {}

trait Dir {
    fn is_enabled(i: usize) -> bool;
}

impl Dir for In {
    fn is_enabled(i: usize) -> bool {
        reg_read(REG_ENDPTCTRL + i * 4) & ENDPTCTRL_TXE != 0
    }
}

impl Dir for Out {
    fn is_enabled(i: usize) -> bool {
        reg_read(REG_ENDPTCTRL + i * 4) & ENDPTCTRL_RXE != 0
    }
}

#[derive(Clone, Copy)]
pub struct Endpoint<'d, D> {
    _phantom: PhantomData<D>,
    ep_state: &'d EndpointState,
    info: EndpointInfo,
}

impl<D> Endpoint<'_, D> {
    fn start_transfer(&mut self) {
        let bit = 1u32 << self.info.addr.index();
        if self.info.addr.is_in() {
            reg_write(REG_ENDPTPRIME, bit << 16);
        } else {
            reg_write(REG_ENDPTPRIME, bit);
        }
    }

    fn transfer(&mut self, data: &[u8]) -> Result<(), TransferError> {
        let ep_num = self.info.addr.index();
        let ep_idx = 2 * ep_num + if self.info.addr.is_in() { 1 } else { 0 };

        if ep_num == 0 {
            while reg_read(REG_ENDPTSETUPSTAT) & 1 != 0 {
                core::hint::spin_loop();
            }
        }

        let qtd_num = (data.len() + 0x3fff) / 0x4000;
        if qtd_num > QTD_COUNT_EACH_QHD {
            return Err(TransferError::BufferTooLarge);
        }
        if data.len() > 0x1000 && (data.as_ptr() as usize) & 0x0fff != 0 {
            return Err(TransferError::BufferAlignment);
        }

        let mut prev_qtd = None;
        let mut first_qtd = None;
        let mut data_offset = 0;
        let mut remaining = data.len();

        for i in 0..qtd_num.max(1) {
            let qtd_idx = ep_idx * QTD_COUNT_EACH_QHD + i;
            let transfer_bytes = remaining.min(0x4000);

            unsafe {
                self.ep_state
                    .qtd_reinit_with(qtd_idx, &data[data_offset..], transfer_bytes);
            }

            if remaining <= 0x4000 {
                unsafe { self.ep_state.qtd_modify(qtd_idx, 1, |v| v | (1 << 15)) };
            }

            if let Some(prev) = prev_qtd {
                let addr = unsafe { self.ep_state.qtd_addr(qtd_idx) };
                unsafe { self.ep_state.qtd_write(prev, 0, addr & !0x1f) };
            } else {
                first_qtd = Some(qtd_idx);
            }

            prev_qtd = Some(qtd_idx);
            data_offset += transfer_bytes;
            remaining = remaining.saturating_sub(transfer_bytes);
            if remaining == 0 {
                break;
            }
        }

        let first_qtd = first_qtd.unwrap();
        unsafe {
            if ep_num == 0 {
                self.ep_state.qhd_modify(ep_idx, 0, |v| v | (1 << 15));
            }
            let qtd_addr = local_to_sys_address(self.ep_state.qtd_addr(first_qtd));
            self.ep_state.qhd_write(ep_idx, 2, qtd_addr & !0x1f);
        }

        self.start_transfer();
        Ok(())
    }

    fn set_stall(&mut self) {
        let reg = REG_ENDPTCTRL + self.info.addr.index() * 4;
        if self.info.addr.is_in() {
            reg_modify(reg, |v| v | ENDPTCTRL_TXS);
        } else {
            reg_modify(reg, |v| v | ENDPTCTRL_RXS);
        }
    }
}

impl<D: Dir> embassy_usb_driver::Endpoint for Endpoint<'_, D> {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        let i = self.info.addr.index();
        if D::is_enabled(i) {
            return;
        }
        poll_fn(|cx| {
            if D::is_enabled(i) {
                Poll::Ready(())
            } else {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await
    }
}

impl EndpointOut for Endpoint<'_, Out> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        let ep_num = self.info.addr.index();
        if !Out::is_enabled(ep_num) {
            return Err(EndpointError::Disabled);
        }

        self.transfer(buf)
            .map_err(|_| EndpointError::BufferOverflow)?;

        poll_fn(|cx| {
            if reg_read(REG_ENDPTCOMPLETE) & (1 << ep_num) != 0 {
                reg_write(REG_ENDPTCOMPLETE, 1 << ep_num);
                Poll::Ready(())
            } else if !Out::is_enabled(ep_num) {
                Poll::Ready(())
            } else {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await;

        let remaining = unsafe { self.ep_state.qhd_total_remaining(2 * ep_num) };
        Ok(buf.len().saturating_sub(remaining))
    }
}

impl EndpointIn for Endpoint<'_, In> {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        let ep_num = self.info.addr.index();
        if !In::is_enabled(ep_num) {
            return Err(EndpointError::Disabled);
        }

        self.transfer(buf)
            .map_err(|_| EndpointError::BufferOverflow)?;

        poll_fn(|cx| {
            if reg_read(REG_ENDPTCOMPLETE) & (1 << (16 + ep_num)) != 0 {
                reg_write(REG_ENDPTCOMPLETE, 1 << (16 + ep_num));
                Poll::Ready(())
            } else if !In::is_enabled(ep_num) {
                Poll::Ready(())
            } else {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await;

        if buf.len() == self.info.max_packet_size as usize {
            self.transfer(&[])
                .map_err(|_| EndpointError::BufferOverflow)?;
            poll_fn(|cx| {
                if reg_read(REG_ENDPTCOMPLETE) & (1 << (16 + ep_num)) != 0 {
                    reg_write(REG_ENDPTCOMPLETE, 1 << (16 + ep_num));
                    Poll::Ready(())
                } else {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            })
            .await;
        }

        Ok(())
    }
}

pub struct ControlPipe<'d> {
    max_packet_size: usize,
    ep_in: Endpoint<'d, In>,
    ep_out: Endpoint<'d, Out>,
}

impl UsbControlPipe for ControlPipe<'_> {
    fn max_packet_size(&self) -> usize {
        self.max_packet_size
    }

    async fn setup(&mut self) -> [u8; 8] {
        debug_mark(2, 0x5301_2000);
        reg_write(REG_USBSTS, USBSTS_UI);
        while reg_read(REG_USBSTS) & USBSTS_UI != 0 {
            core::hint::spin_loop();
        }
        reg_modify(REG_USBINTR, |v| v | USBINTR_UE);

        poll_fn(|cx| {
            if reg_read(REG_ENDPTSETUPSTAT) & 1 != 0 {
                debug_mark(2, 0x5301_2001);
                reg_write(REG_ENDPTSETUPSTAT, 1);
                reg_write(REG_ENDPTCOMPLETE, 1);
                Poll::Ready(())
            } else {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await;

        reg_write(REG_USBSTS, USBSTS_UI);
        while reg_read(REG_USBSTS) & USBSTS_UI != 0 {
            core::hint::spin_loop();
        }
        reg_modify(REG_USBINTR, |v| v | USBINTR_UE);
        let request = unsafe { self.ep_out.ep_state.setup_request() };
        debug_mark(2, 0x5301_2002);
        debug_mark(
            3,
            u32::from_le_bytes([request[0], request[1], request[2], request[3]]),
        );
        debug_mark(
            4,
            u32::from_le_bytes([request[4], request[5], request[6], request[7]]),
        );
        request
    }

    async fn data_out(
        &mut self,
        buf: &mut [u8],
        _first: bool,
        _last: bool,
    ) -> Result<usize, EndpointError> {
        self.ep_out
            .transfer(buf)
            .map_err(|_| EndpointError::BufferOverflow)?;

        poll_fn(|cx| {
            if reg_read(REG_ENDPTCOMPLETE) & 1 != 0 {
                reg_write(REG_ENDPTCOMPLETE, 1);
                Poll::Ready(())
            } else {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await;

        let remaining = unsafe { self.ep_out.ep_state.qhd_total_remaining(0) };
        Ok(buf.len().saturating_sub(remaining))
    }

    async fn data_in(
        &mut self,
        data: &[u8],
        _first: bool,
        last: bool,
    ) -> Result<(), EndpointError> {
        debug_mark(5, 0x5301_3000 | (data.len() as u32 & 0xff));
        self.ep_in
            .transfer(data)
            .map_err(|_| EndpointError::BufferOverflow)?;
        poll_fn(|cx| {
            if reg_read(REG_ENDPTCOMPLETE) & (1 << 16) != 0 {
                reg_write(REG_ENDPTCOMPLETE, 1 << 16);
                Poll::Ready(())
            } else {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await;
        debug_mark(5, 0x5301_3001);
        if last {
            let _ = self.ep_out.transfer(&[]);
            poll_fn(|cx| {
                if reg_read(REG_ENDPTCOMPLETE) & 1 != 0 {
                    reg_write(REG_ENDPTCOMPLETE, 1);
                    Poll::Ready(())
                } else {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            })
            .await;
        }
        debug_mark(5, 0x5301_3002);
        Ok(())
    }

    async fn accept(&mut self) {
        debug_mark(5, 0x5301_3100);
        let _ = self.ep_in.transfer(&[]);
        poll_fn(|cx| {
            if reg_read(REG_ENDPTCOMPLETE) & (1 << 16) != 0 {
                reg_write(REG_ENDPTCOMPLETE, 1 << 16);
                Poll::Ready(())
            } else {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await;
        debug_mark(5, 0x5301_3101);
    }

    async fn reject(&mut self) {
        self.ep_in.set_stall();
        self.ep_out.set_stall();
    }

    async fn accept_set_address(&mut self, addr: u8) {
        debug_mark(5, 0x5301_3200 | u32::from(addr));
        reg_write(REG_DEVICEADDR, (u32::from(addr & 0x7f) << 25) | (1 << 24));
        self.accept().await;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransferError {
    BufferTooLarge,
    BufferAlignment,
}

#[inline(always)]
fn reg_read(offset: usize) -> u32 {
    unsafe { ptr::read_volatile((USB_BASE + offset) as *const u32) }
}

#[inline(always)]
fn reg_write(offset: usize, value: u32) {
    unsafe { ptr::write_volatile((USB_BASE + offset) as *mut u32, value) };
}

#[inline(always)]
fn reg_modify(offset: usize, f: impl FnOnce(u32) -> u32) {
    let value = reg_read(offset);
    reg_write(offset, f(value));
}

#[inline(always)]
fn debug_mark(_slot: usize, _value: u32) {}

#[inline(always)]
fn local_to_sys_address(addr: u32) -> u32 {
    addr
}

fn configure_usb_pins() {
    let ioc = unsafe { &*pac::Ioc::ptr() };
    unsafe {
        ioc.pad(24)
            .func_ctl()
            .write(|w| w.alt_select().bits(0).analog().set_bit());
        ioc.pad(25)
            .func_ctl()
            .write(|w| w.alt_select().bits(0).analog().set_bit());
        ioc.pad(448)
            .func_ctl()
            .write(|w| w.alt_select().bits(25).analog().clear_bit());
        ioc.pad(449)
            .func_ctl()
            .write(|w| w.alt_select().bits(25).analog().clear_bit());
    }
}
