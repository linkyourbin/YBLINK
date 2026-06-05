#![no_std]
#![no_main]

mod boot;
mod dap;
mod fast_gpio;
mod serial;
mod swj;
mod usb_dap;

use core::cell::UnsafeCell;
use core::future::{Future, poll_fn};
use core::pin::Pin;
use core::task::Poll;

use dap::{DAP_PRODUCT, DAP_SERIAL, DAP_VENDOR, Dap, PACKET_COUNT, PACKET_SIZE};
use embassy_executor::Spawner;
use embassy_futures::join::join3;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State as CdcAcmState};
use embassy_usb::driver::{
    Bus, Driver, EndpointAddress, EndpointAllocError, EndpointType, Event, Unsupported,
};
use embassy_usb::{Builder, msos};
use fast_gpio::ProbePins;
use hal::usb::{EndpointState, UsbDriver};
use hpm5301_hal as hal;
use panic_halt as _;
use serial::{SERIAL_USB_PACKET_SIZE, SerialUart, UART_RX_DMA_BUFFER_SIZE};
use swj::Swj;
use usb_dap::{CmsisDapV2Class, State as CmsisDapState};

#[unsafe(link_section = ".noncacheable")]
static EP_STATE: EndpointState = EndpointState::new();

#[unsafe(link_section = ".noncacheable")]
static CONFIG_DESC: StaticCell<[u8; 1024]> = StaticCell::new([0; 1024]);
#[unsafe(link_section = ".noncacheable")]
static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new([0; 256]);
#[unsafe(link_section = ".noncacheable")]
static MSOS_DESC: StaticCell<[u8; 1024]> = StaticCell::new([0; 1024]);
#[unsafe(link_section = ".noncacheable")]
static CONTROL_BUF: StaticCell<[u8; 256]> = StaticCell::new([0; 256]);
static CMSIS_DAP_STATE: StaticCell<CmsisDapState> = StaticCell::new(CmsisDapState::new());
static CDC_ACM_STATE: StaticCell<CdcAcmState<'static>> = StaticCell::new(CdcAcmState::new());
#[unsafe(link_section = ".noncacheable")]
static REQUEST_BUF: StaticCell<PacketBuffers<PACKET_SIZE, PACKET_COUNT>> =
    StaticCell::new(PacketBuffers::new());
#[unsafe(link_section = ".noncacheable")]
static RESPONSE_BUF: StaticCell<PacketBuffers<PACKET_SIZE, PACKET_COUNT>> =
    StaticCell::new(PacketBuffers::new());
#[unsafe(link_section = ".noncacheable")]
static SERIAL_USB_OUT_BUF: StaticCell<[u8; SERIAL_USB_PACKET_SIZE]> =
    StaticCell::new([0; SERIAL_USB_PACKET_SIZE]);
#[unsafe(link_section = ".noncacheable")]
static SERIAL_USB_IN_BUF: StaticCell<[u8; SERIAL_USB_PACKET_SIZE]> =
    StaticCell::new([0; SERIAL_USB_PACKET_SIZE]);
#[unsafe(link_section = ".noncacheable")]
static UART_RX_DMA_BUF: StaticCell<[u8; UART_RX_DMA_BUFFER_SIZE]> =
    StaticCell::new([0; UART_RX_DMA_BUFFER_SIZE]);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    boot_mark(0x5301_0001);
    let p = hal::init(Default::default());
    boot_mark(0x5301_0002);

    let usb_driver_config = hal::usb::Config::default();
    let driver = UsbDriver::new(p.USB0, p.PA24, p.PA25, usb_driver_config, &EP_STATE);
    boot_mark(0x5301_0003);
    let driver = ForcePoweredDriver::new(driver);
    boot_mark(0x5301_0031);

    let mut config = embassy_usb::Config::new(0x1209, 0x5301);
    config.manufacturer = Some(DAP_VENDOR);
    config.product = Some(DAP_PRODUCT);
    config.serial_number = Some(DAP_SERIAL);
    config.device_release = 0x003d;
    config.max_power = 250;
    config.max_packet_size_0 = 64;
    boot_mark(0x5301_0032);

    boot_mark(0x5301_0033);
    let mut builder = Builder::new(
        driver,
        config,
        CONFIG_DESC.get(),
        BOS_DESC.get(),
        MSOS_DESC.get(),
        CONTROL_BUF.get(),
    );
    boot_mark(0x5301_0034);
    builder.msos_descriptor(msos::windows_version::WIN8_1, 0x20);
    builder.msos_feature(msos::CcgpDeviceDescriptor::new());
    boot_mark(0x5301_0036);

    let cmsis_dap = CmsisDapV2Class::new(&mut builder, CMSIS_DAP_STATE.get(), 512, true);
    let cdc_acm = CdcAcmClass::new(
        &mut builder,
        CDC_ACM_STATE.get(),
        SERIAL_USB_PACKET_SIZE as u16,
    );
    boot_mark(0x5301_0035);
    let mut usb = builder.build();
    boot_mark(0x5301_0004);

    let (serial_usb_tx, serial_usb_rx) = cdc_acm.split();
    let serial = SerialUart::new(
        p.UART0,
        p.PA00,
        p.PA01,
        p.HDMA,
        p.DMAMUX,
        p.HDMA_CH0,
        UART_RX_DMA_BUF.get(),
    );
    let (uart_tx, uart_rx) = serial.split();

    let requests = REQUEST_BUF.get().as_mut();
    let responses = RESPONSE_BUF.get().as_mut();

    let usb_fut = usb.run();
    let dap_fut = async move {
        let (mut dap_reader, mut dap_writer) = cmsis_dap.split();
        dap_reader.wait_connection().await;

        let pins = ProbePins::new(p.PA26, p.PA27, p.PA28, p.PA29, p.PB10);
        let mut dap = Dap::new(Swj::new(pins));

        loop {
            let mut current = 0usize;
            let mut len = match dap_reader.read_packet(&mut requests[current]).await {
                Ok(len) => len,
                Err(_) => {
                    dap_reader.wait_connection().await;
                    continue;
                }
            };

            loop {
                let next = (current + 1) % PACKET_COUNT;
                let (request, next_request) = packet_pair(requests, current, next);
                let mut next_read = dap_reader.read_packet(next_request);
                let next_read = unsafe { Pin::new_unchecked(&mut next_read) };
                let mut next_read = next_read;
                let _ = poll_once(next_read.as_mut()).await;

                let response_len = dap.process(&request[..len], &mut responses[current]);
                if response_len == 0 {
                    match next_read.await {
                        Ok(next_len) => {
                            current = next;
                            len = next_len;
                            continue;
                        }
                        Err(_) => break,
                    }
                }

                if dap_writer
                    .write_packet(&responses[current][..response_len])
                    .await
                    .is_err()
                {
                    break;
                }

                match next_read.await {
                    Ok(next_len) => {
                        current = next;
                        len = next_len;
                    }
                    Err(_) => break,
                }
            }

            dap_reader.wait_connection().await;
        }
    };
    let serial_fut = serial::run(
        serial_usb_tx,
        serial_usb_rx,
        uart_tx,
        uart_rx,
        SERIAL_USB_OUT_BUF.get(),
        SERIAL_USB_IN_BUF.get(),
    );

    let _ = join3(usb_fut, dap_fut, serial_fut).await;
}

async fn poll_once<F: Future>(mut future: Pin<&mut F>) -> Option<F::Output> {
    poll_fn(|cx| match future.as_mut().poll(cx) {
        Poll::Ready(value) => Poll::Ready(Some(value)),
        Poll::Pending => Poll::Ready(None),
    })
    .await
}

fn packet_pair<const N: usize>(
    packets: &mut [[u8; PACKET_SIZE]; N],
    current: usize,
    next: usize,
) -> (&[u8; PACKET_SIZE], &mut [u8; PACKET_SIZE]) {
    debug_assert!(current < N);
    debug_assert!(next < N);
    debug_assert!(current != next);

    if current < next {
        let (left, right) = packets.split_at_mut(next);
        (&left[current], &mut right[0])
    } else {
        let (left, right) = packets.split_at_mut(current);
        (&right[0], &mut left[next])
    }
}

#[inline(always)]
fn boot_mark(_value: u32) {}

struct StaticCell<T>(UnsafeCell<T>);

unsafe impl<T> Sync for StaticCell<T> {}

impl<T> StaticCell<T> {
    const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    fn get(&'static self) -> &'static mut T {
        unsafe { &mut *self.0.get() }
    }
}

#[repr(align(4096))]
struct PacketBuffers<const SIZE: usize, const COUNT: usize>([[u8; SIZE]; COUNT]);

impl<const SIZE: usize, const COUNT: usize> PacketBuffers<SIZE, COUNT> {
    const fn new() -> Self {
        Self([[0; SIZE]; COUNT])
    }

    fn as_mut(&mut self) -> &mut [[u8; SIZE]; COUNT] {
        &mut self.0
    }
}

struct ForcePoweredDriver<D> {
    inner: D,
}

impl<D> ForcePoweredDriver<D> {
    fn new(inner: D) -> Self {
        Self { inner }
    }
}

impl<'d, D> Driver<'d> for ForcePoweredDriver<D>
where
    D: Driver<'d>,
{
    type EndpointOut = D::EndpointOut;
    type EndpointIn = D::EndpointIn;
    type ControlPipe = D::ControlPipe;
    type Bus = ForcePoweredBus<D::Bus>;

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, EndpointAllocError> {
        self.inner
            .alloc_endpoint_out(ep_type, ep_addr, max_packet_size, interval_ms)
    }

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, EndpointAllocError> {
        self.inner
            .alloc_endpoint_in(ep_type, ep_addr, max_packet_size, interval_ms)
    }

    fn start(self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        let (bus, control_pipe) = self.inner.start(control_max_packet_size);
        (
            ForcePoweredBus {
                inner: bus,
                power_detected_sent: false,
            },
            control_pipe,
        )
    }
}

struct ForcePoweredBus<B> {
    inner: B,
    power_detected_sent: bool,
}

impl<B> Bus for ForcePoweredBus<B>
where
    B: Bus,
{
    async fn enable(&mut self) {
        boot_mark(0x5301_0005);
        self.inner.enable().await;
        boot_mark(0x5301_0006);
    }

    async fn disable(&mut self) {
        self.inner.disable().await;
    }

    async fn poll(&mut self) -> Event {
        boot_mark(0x5301_0600);
        if !self.power_detected_sent {
            self.power_detected_sent = true;
            boot_mark(0x5301_0601);
            return Event::PowerDetected;
        }

        loop {
            boot_mark(0x5301_0602);
            match self.inner.poll().await {
                Event::PowerRemoved => {
                    boot_mark(0x5301_06ff);
                }
                Event::PowerDetected => {
                    boot_mark(0x5301_0603);
                    return Event::PowerDetected;
                }
                Event::Reset => {
                    boot_mark(0x5301_0604);
                    return Event::Reset;
                }
                Event::Suspend => {
                    boot_mark(0x5301_0605);
                    return Event::Suspend;
                }
                Event::Resume => {
                    boot_mark(0x5301_0606);
                    return Event::Resume;
                }
            }
        }
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        self.inner.endpoint_set_enabled(ep_addr, enabled);
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        self.inner.endpoint_set_stalled(ep_addr, stalled);
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        self.inner.endpoint_is_stalled(ep_addr)
    }

    fn force_reset(&mut self) -> Result<(), Unsupported> {
        self.inner.force_reset()
    }

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        self.inner.remote_wakeup().await
    }
}
