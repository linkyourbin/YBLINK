use core::mem::MaybeUninit;

use embassy_usb::driver::{Driver, Endpoint, EndpointError, EndpointIn, EndpointOut};
use embassy_usb::types::StringIndex;
use embassy_usb::{Builder, Handler, msos};

pub struct State {
    control: MaybeUninit<Control>,
}

struct Control {
    iface_string: StringIndex,
}

impl Handler for Control {
    fn get_string(&mut self, index: StringIndex, _lang_id: u16) -> Option<&str> {
        if index == self.iface_string {
            Some("YBLINK CMSIS-DAP v2 Interface")
        } else {
            None
        }
    }
}

impl State {
    pub const fn new() -> Self {
        Self {
            control: MaybeUninit::uninit(),
        }
    }
}

pub struct CmsisDapV2Class<'d, D: Driver<'d>> {
    read_ep: D::EndpointOut,
    write_ep: D::EndpointIn,
    #[allow(dead_code)]
    trace_ep: Option<D::EndpointIn>,
    max_packet_size: u16,
}

impl<'d, D: Driver<'d>> CmsisDapV2Class<'d, D> {
    pub fn new(
        builder: &mut Builder<'d, D>,
        state: &'d mut State,
        max_packet_size: u16,
        trace: bool,
    ) -> Self {
        let iface_string = builder.string();
        let mut function = builder.function(0xff, 0, 0);
        function.msos_feature(msos::CompatibleIdFeatureDescriptor::new("WINUSB", ""));
        function.msos_feature(msos::RegistryPropertyFeatureDescriptor::new(
            "DeviceInterfaceGUIDs",
            msos::PropertyData::RegMultiSz(&["{CDB3B5AD-293B-4663-AA36-1AAE46463776}"]),
        ));
        let mut interface = function.interface();
        let mut alt = interface.alt_setting(0xff, 0, 0, Some(iface_string));
        let read_ep = alt.endpoint_bulk_out(None, max_packet_size);
        let write_ep = alt.endpoint_bulk_in(None, max_packet_size);
        let trace_ep = if trace {
            Some(alt.endpoint_bulk_in(None, max_packet_size))
        } else {
            None
        };
        drop(function);

        builder.handler(state.control.write(Control { iface_string }));

        Self {
            read_ep,
            write_ep,
            trace_ep,
            max_packet_size,
        }
    }

    pub fn split(self) -> (CmsisDapV2Reader<'d, D>, CmsisDapV2Writer<'d, D>) {
        (
            CmsisDapV2Reader {
                read_ep: self.read_ep,
            },
            CmsisDapV2Writer {
                write_ep: self.write_ep,
                trace_ep: self.trace_ep,
                max_packet_size: self.max_packet_size,
            },
        )
    }

    #[allow(dead_code)]
    pub async fn write_trace(&mut self, data: &[u8]) -> Result<(), EndpointError> {
        let Some(ep) = self.trace_ep.as_mut() else {
            return Err(EndpointError::Disabled);
        };

        ep.write(data).await?;
        if data.len() > self.max_packet_size as usize
            && data.len() % self.max_packet_size as usize == 0
        {
            ep.write(&[]).await?;
        }
        Ok(())
    }
}

pub struct CmsisDapV2Reader<'d, D: Driver<'d>> {
    read_ep: D::EndpointOut,
}

impl<'d, D: Driver<'d>> CmsisDapV2Reader<'d, D> {
    pub async fn wait_connection(&mut self) {
        self.read_ep.wait_enabled().await;
    }

    pub async fn read_packet(&mut self, data: &mut [u8]) -> Result<usize, EndpointError> {
        self.read_ep.read(data).await
    }
}

pub struct CmsisDapV2Writer<'d, D: Driver<'d>> {
    write_ep: D::EndpointIn,
    #[allow(dead_code)]
    trace_ep: Option<D::EndpointIn>,
    max_packet_size: u16,
}

impl<'d, D: Driver<'d>> CmsisDapV2Writer<'d, D> {
    pub async fn write_packet(&mut self, data: &[u8]) -> Result<(), EndpointError> {
        self.write_ep.write(data).await?;
        if data.len() > self.max_packet_size as usize
            && data.len() % self.max_packet_size as usize == 0
        {
            self.write_ep.write(&[]).await?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn write_trace(&mut self, data: &[u8]) -> Result<(), EndpointError> {
        let Some(ep) = self.trace_ep.as_mut() else {
            return Err(EndpointError::Disabled);
        };

        ep.write(data).await?;
        if data.len() > self.max_packet_size as usize
            && data.len() % self.max_packet_size as usize == 0
        {
            ep.write(&[]).await?;
        }
        Ok(())
    }
}
