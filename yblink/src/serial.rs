use core::marker::PhantomData;
use core::sync::atomic::{Ordering, compiler_fence};

use embassy_futures::join::join;
use embassy_time::{Duration, Timer};
use embassy_usb::class::cdc_acm::{Receiver, Sender};
use embassy_usb::driver::{Driver, EndpointError};
use hpm5301_hal::sysctl::{self, Resource};
use hpm5301_hal::{Peri, pac, peripherals};

pub const DEFAULT_BAUD: u32 = 1_000_000;
pub const UART_RX_DMA_BUFFER_SIZE: usize = 64 * 1024;
pub const SERIAL_USB_PACKET_SIZE: usize = 512;

const UART_CLOCK_HZ: u32 = 400_000_000;
const UART0_RBR_ADDR: u32 = 0xf004_0020;
const DMA_CHANNEL: usize = 0;
const DMA_CHANNEL_MASK: u32 = 1 << DMA_CHANNEL;
const DMA_REQ_UART0_RX: u8 = 0x14;
const UART_TX_FIFO_DEPTH: usize = 16;

pub struct SerialUart {
    rx_ptr: *mut u8,
    rx_len: usize,
    _owned: PhantomData<(
        peripherals::UART0,
        peripherals::PA00,
        peripherals::PA01,
        peripherals::HDMA,
        peripherals::DMAMUX,
        peripherals::HDMA_CH0,
    )>,
}

pub struct SerialTx {
    baud: u32,
}

pub struct SerialRx {
    rx_ptr: *mut u8,
    rx_len: usize,
    tail: usize,
}

impl SerialUart {
    pub fn new(
        _uart0: Peri<'static, peripherals::UART0>,
        _tx: Peri<'static, peripherals::PA00>,
        _rx: Peri<'static, peripherals::PA01>,
        _hdma: Peri<'static, peripherals::HDMA>,
        _dmamux: Peri<'static, peripherals::DMAMUX>,
        _hdma_ch0: Peri<'static, peripherals::HDMA_CH0>,
        rx_dma_buffer: &'static mut [u8; UART_RX_DMA_BUFFER_SIZE],
    ) -> Self {
        let rx_ptr = rx_dma_buffer.as_mut_ptr();
        let rx_len = rx_dma_buffer.len();

        configure_uart0(DEFAULT_BAUD);
        configure_rx_dma(rx_ptr, rx_len);

        Self {
            rx_ptr,
            rx_len,
            _owned: PhantomData,
        }
    }

    pub fn split(self) -> (SerialTx, SerialRx) {
        (
            SerialTx { baud: DEFAULT_BAUD },
            SerialRx {
                rx_ptr: self.rx_ptr,
                rx_len: self.rx_len,
                tail: 0,
            },
        )
    }
}

impl SerialTx {
    pub fn set_baud(&mut self, baud: u32) {
        if valid_baud(baud) && baud != self.baud {
            set_uart0_baud(baud);
            self.baud = baud;
        }
    }

    pub fn write_blocking(&mut self, mut data: &[u8]) {
        let uart = unsafe { &*pac::Uart0::ptr() };

        while !data.is_empty() {
            if uart.lsr().read().tfifo_num().bits() as usize >= UART_TX_FIFO_DEPTH {
                core::hint::spin_loop();
                continue;
            }

            unsafe {
                uart.union_20_thr().write(|w| w.thr().bits(data[0]));
            }
            data = &data[1..];
        }
    }
}

impl SerialRx {
    pub fn discard_pending(&mut self) {
        self.tail = self.dma_head();
    }

    pub fn read_available(&mut self, out: &mut [u8]) -> usize {
        if out.is_empty() {
            return 0;
        }

        compiler_fence(Ordering::Acquire);
        let head = self.dma_head();
        if head == self.tail {
            return 0;
        }

        let n = if head > self.tail {
            (head - self.tail).min(out.len())
        } else {
            (self.rx_len - self.tail).min(out.len())
        };

        unsafe {
            let src = core::slice::from_raw_parts(self.rx_ptr.add(self.tail), n);
            out[..n].copy_from_slice(src);
        }

        self.tail = (self.tail + n) % self.rx_len;
        n
    }

    fn dma_head(&self) -> usize {
        let hdma = unsafe { &*pac::Hdma::ptr() };
        let remaining = hdma
            .chctrl(DMA_CHANNEL)
            .tran_size()
            .read()
            .transize()
            .bits() as usize;

        if remaining == 0 || remaining > self.rx_len {
            0
        } else {
            self.rx_len - remaining
        }
    }
}

pub async fn run<'d, D: Driver<'d>>(
    usb_tx: Sender<'d, D>,
    usb_rx: Receiver<'d, D>,
    uart_tx: SerialTx,
    uart_rx: SerialRx,
    usb_out_buf: &'static mut [u8; SERIAL_USB_PACKET_SIZE],
    usb_in_buf: &'static mut [u8; SERIAL_USB_PACKET_SIZE],
) {
    let rx_fut = usb_to_uart(usb_rx, uart_tx, usb_out_buf);
    let tx_fut = uart_to_usb(usb_tx, uart_rx, usb_in_buf);
    let _ = join(rx_fut, tx_fut).await;
}

async fn usb_to_uart<'d, D: Driver<'d>>(
    mut usb_rx: Receiver<'d, D>,
    mut uart_tx: SerialTx,
    buf: &'static mut [u8; SERIAL_USB_PACKET_SIZE],
) {
    loop {
        usb_rx.wait_connection().await;

        loop {
            match usb_rx.read_packet(buf).await {
                Ok(len) => {
                    uart_tx.set_baud(usb_rx.line_coding().data_rate());
                    uart_tx.write_blocking(&buf[..len]);
                    Timer::after(Duration::from_micros(1)).await;
                }
                Err(EndpointError::Disabled) => break,
                Err(EndpointError::BufferOverflow) => {}
            }
        }
    }
}

async fn uart_to_usb<'d, D: Driver<'d>>(
    mut usb_tx: Sender<'d, D>,
    mut uart_rx: SerialRx,
    buf: &'static mut [u8; SERIAL_USB_PACKET_SIZE],
) {
    let mut current_baud = DEFAULT_BAUD;
    loop {
        usb_tx.wait_connection().await;
        uart_rx.discard_pending();

        loop {
            let baud = usb_tx.line_coding().data_rate();
            if valid_baud(baud) && baud != current_baud {
                set_uart0_baud(baud);
                current_baud = baud;
            }

            let len = uart_rx.read_available(buf);
            if len == 0 {
                Timer::after(Duration::from_micros(100)).await;
                continue;
            }

            match usb_tx.write_packet(&buf[..len]).await {
                Ok(()) => {}
                Err(EndpointError::Disabled) => break,
                Err(EndpointError::BufferOverflow) => {}
            }
        }
    }
}

fn configure_uart0(baud: u32) {
    sysctl::enable_resource(Resource::Urt0);
    configure_uart0_clock();
    configure_uart0_pins();

    let uart = unsafe { &*pac::Uart0::ptr() };

    unsafe {
        uart.union_24_ier().write(|w| w.bits(0));
        set_uart0_baud_unchecked(baud);
        uart.fcrr()
            .write(|w| w.rfiforst().set_bit().tfiforst().set_bit());
        uart.fcrr().write(|w| {
            w.fifoe()
                .set_bit()
                .dmae()
                .set_bit()
                .fifot4en()
                .set_bit()
                .rfifot4()
                .bits(0)
                .tfifot4()
                .bits(15)
        });
        uart.idle_cfg().write(|w| {
            w.rxen()
                .set_bit()
                .rx_idle_thr()
                .bits(10)
                .rx_idle_en()
                .clear_bit()
                .rx_idle_cond()
                .set_bit()
        });
    }

    let _ = uart.lsr().read();
}

fn set_uart0_baud(baud: u32) {
    if valid_baud(baud) {
        let uart = unsafe { &*pac::Uart0::ptr() };
        while uart.lsr().read().temt().bit_is_clear() {
            core::hint::spin_loop();
        }
        unsafe {
            set_uart0_baud_unchecked(baud);
        }
        let _ = uart.lsr().read();
    }
}

unsafe fn set_uart0_baud_unchecked(baud: u32) {
    let uart = unsafe { &*pac::Uart0::ptr() };
    let (div, osc) = calculate_baudrate(UART_CLOCK_HZ, baud);

    unsafe {
        uart.lcr().write(|w| w.dlab().set_bit());
        uart.oscr().write(|w| w.osc().bits(osc));
        uart.union_20_dll().write(|w| w.dll().bits(div as u8));
        uart.union_24_dlm()
            .write(|w| w.dlm().bits((div >> 8) as u8));
        uart.lcr().write(|w| {
            w.dlab()
                .clear_bit()
                .wls()
                .bits(3)
                .stb()
                .clear_bit()
                .pen()
                .clear_bit()
                .eps()
                .clear_bit()
                .sps()
                .clear_bit()
                .bc()
                .clear_bit()
        });
    }
}

fn configure_uart0_clock() {
    let sysctl = unsafe { &*pac::Sysctl::ptr() };

    unsafe {
        sysctl
            .clockclk_top_urt0()
            .write(|w| w.mux().bits(4).div().bits(1).preserve().set_bit());
    }

    while sysctl.clockclk_top_urt0().read().loc_busy().bit_is_set() {
        core::hint::spin_loop();
    }
}

fn configure_uart0_pins() {
    configure_ioc_pad(0, 2, false);
    configure_ioc_pad(1, 2, true);
}

fn configure_ioc_pad(index: usize, alt: u8, pull_up: bool) {
    let ioc = unsafe { &*pac::Ioc::ptr() };
    let pad = ioc.pad(index);

    unsafe {
        pad.func_ctl().write(|w| {
            w.alt_select()
                .bits(alt)
                .analog()
                .clear_bit()
                .loop_back()
                .clear_bit()
        });

        pad.pad_ctl().write(|w| {
            w.ds()
                .bits(7)
                .spd()
                .bits(3)
                .sr()
                .set_bit()
                .od()
                .clear_bit()
                .ke()
                .clear_bit()
                .pe()
                .bit(pull_up)
                .ps()
                .bit(pull_up)
                .prs()
                .bits(0)
                .hys()
                .bit(pull_up)
        });
    }
}

fn configure_rx_dma(rx_ptr: *mut u8, rx_len: usize) {
    sysctl::enable_resource(Resource::Hdma);

    let dmamux = unsafe { &*pac::Dmamux::ptr() };
    let hdma = unsafe { &*pac::Hdma::ptr() };
    let ch = hdma.chctrl(DMA_CHANNEL);

    unsafe {
        ch.ctrl().write(|w| w.enable().clear_bit());
        hdma.ch_abort()
            .write(|w| w.chabort().bits(DMA_CHANNEL_MASK));
        hdma.inthalfsts().write(|w| w.sts().bits(DMA_CHANNEL_MASK));
        hdma.inttcsts().write(|w| w.sts().bits(DMA_CHANNEL_MASK));
        hdma.intabortsts().write(|w| w.sts().bits(DMA_CHANNEL_MASK));
        hdma.interrsts().write(|w| w.sts().bits(DMA_CHANNEL_MASK));

        dmamux
            .muxcfg(DMA_CHANNEL)
            .write(|w| w.source().bits(DMA_REQ_UART0_RX).enable().set_bit());

        ch.src_addr().write(|w| w.srcaddrl().bits(UART0_RBR_ADDR));
        ch.dst_addr().write(|w| w.dstaddrl().bits(rx_ptr as u32));
        ch.tran_size().write(|w| w.transize().bits(rx_len as u32));
        ch.llpointer().write(|w| w.bits(0));
        ch.chan_req_ctrl()
            .write(|w| w.srcreqsel().bits(DMA_CHANNEL as u8));
        ch.ctrl().write(|w| {
            w.inttcmask()
                .set_bit()
                .interrmask()
                .clear_bit()
                .intabtmask()
                .set_bit()
                .inthalfcntmask()
                .set_bit()
                .dstaddrctrl()
                .bits(0)
                .srcaddrctrl()
                .bits(2)
                .dstmode()
                .clear_bit()
                .srcmode()
                .set_bit()
                .dstwidth()
                .bits(0)
                .srcwidth()
                .bits(0)
                .srcburstsize()
                .bits(0)
                .burstopt()
                .set_bit()
                .priority()
                .set_bit()
                .handshakeopt()
                .clear_bit()
                .infiniteloop()
                .set_bit()
                .enable()
                .set_bit()
        });
    }
}

fn valid_baud(baudrate: u32) -> bool {
    (200..=50_000_000).contains(&baudrate)
}

fn calculate_baudrate(freq: u32, baudrate: u32) -> (u16, u8) {
    let mut best_div = 1u16;
    let mut best_osc = 8u8;
    let mut best_error = u32::MAX;

    for osc in (8..=32).step_by(2) {
        let denom = baudrate.saturating_mul(osc);
        let div = ((freq + denom / 2) / denom).clamp(1, u16::MAX as u32);
        let actual = freq / (div * osc);
        let error = actual.abs_diff(baudrate);

        if error < best_error {
            best_error = error;
            best_div = div as u16;
            best_osc = if osc == 32 { 0 } else { osc as u8 };
        }
    }

    (best_div, best_osc)
}
