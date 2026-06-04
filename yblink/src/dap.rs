use hpm5301_hal::time;

use crate::swj::{Port, ProbeIo, Swj, TransferStatus};

pub const PACKET_SIZE: usize = 512;
pub const DAP_VENDOR: &str = "linkyourbin";
pub const DAP_PRODUCT: &str = "YBLINK CMSIS-DAP";
pub const DAP_SERIAL: &str = "YBLINK";
pub const DAP_FW_VERSION: &str = "0.1.0";
pub const DAP_PRODUCT_FW_VERSION: &str = "0.1.0";

#[unsafe(no_mangle)]
#[used]
pub static mut YBLINK_DAP_TRACE: [u32; 1024] = [0; 1024];
static mut DAP_TRACE_INDEX: u32 = 0;
const ENABLE_DAP_TRACE: bool = false;

const DAP_OK: u8 = 0x00;
const DAP_ERROR: u8 = 0xff;

const DAP_TRANSFER_OK: u8 = 1 << 0;
const DAP_TRANSFER_WAIT: u8 = 1 << 1;
const DAP_TRANSFER_FAULT: u8 = 1 << 2;
const DAP_TRANSFER_NO_ACK: u8 = 0x07;
const DAP_TRANSFER_ERROR: u8 = 1 << 3;
const DAP_TRANSFER_MISMATCH: u8 = 1 << 4;
const DAP_TRANSFER_APNDP: u8 = 1 << 0;
const DAP_TRANSFER_RNW: u8 = 1 << 1;
const DAP_TRANSFER_MATCH_VALUE: u8 = 1 << 4;
const DAP_TRANSFER_MATCH_MASK: u8 = 1 << 5;
const DP_RDBUFF_READ: u8 = 0x0e;
const CHECK_POSTED_WRITES: bool = true;
const JTAG_DEV_MAX: usize = 8;
const JTAG_ABORT: u8 = 0x08;
const JTAG_DPACC: u8 = 0x0a;
const JTAG_APACC: u8 = 0x0b;
const JTAG_IDCODE: u8 = 0x0e;

const ID_DAP_INFO: u8 = 0x00;
const ID_DAP_HOST_STATUS: u8 = 0x01;
const ID_DAP_CONNECT: u8 = 0x02;
const ID_DAP_DISCONNECT: u8 = 0x03;
const ID_DAP_TRANSFER_CONFIGURE: u8 = 0x04;
const ID_DAP_TRANSFER: u8 = 0x05;
const ID_DAP_TRANSFER_BLOCK: u8 = 0x06;
const ID_DAP_TRANSFER_ABORT: u8 = 0x07;
const ID_DAP_WRITE_ABORT: u8 = 0x08;
const ID_DAP_DELAY: u8 = 0x09;
const ID_DAP_RESET_TARGET: u8 = 0x0a;
const ID_DAP_SWJ_PINS: u8 = 0x10;
const ID_DAP_SWJ_CLOCK: u8 = 0x11;
const ID_DAP_SWJ_SEQUENCE: u8 = 0x12;
const ID_DAP_SWD_CONFIGURE: u8 = 0x13;
const ID_DAP_JTAG_SEQUENCE: u8 = 0x14;
const ID_DAP_JTAG_CONFIGURE: u8 = 0x15;
const ID_DAP_JTAG_IDCODE: u8 = 0x16;
const ID_DAP_SWO_TRANSPORT: u8 = 0x17;
const ID_DAP_SWO_MODE: u8 = 0x18;
const ID_DAP_SWO_BAUDRATE: u8 = 0x19;
const ID_DAP_SWO_CONTROL: u8 = 0x1a;
const ID_DAP_SWO_STATUS: u8 = 0x1b;
const ID_DAP_SWO_DATA: u8 = 0x1c;
const ID_DAP_SWD_SEQUENCE: u8 = 0x1d;
const ID_DAP_SWO_EXTENDED_STATUS: u8 = 0x1e;
const ID_DAP_QUEUE_COMMANDS: u8 = 0x7e;
const ID_DAP_EXECUTE_COMMANDS: u8 = 0x7f;

pub struct Dap<P: ProbeIo> {
    swj: Swj<P>,
    port: Port,
    jtag_device_count: u8,
    jtag_index: u8,
    jtag_ir_length: [u8; JTAG_DEV_MAX],
    jtag_ir_before: [u16; JTAG_DEV_MAX],
    jtag_ir_after: [u16; JTAG_DEV_MAX],
    wait_retries: usize,
    match_retries: usize,
    match_mask: u32,
}

impl<P: ProbeIo> Dap<P> {
    pub fn new(swj: Swj<P>) -> Self {
        Self {
            swj,
            port: Port::Disabled,
            jtag_device_count: 0,
            jtag_index: 0,
            jtag_ir_length: [0; JTAG_DEV_MAX],
            jtag_ir_before: [0; JTAG_DEV_MAX],
            jtag_ir_after: [0; JTAG_DEV_MAX],
            wait_retries: 100,
            match_retries: 0,
            match_mask: 0,
        }
    }

    pub fn process(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        let Some(&command) = request.first() else {
            return 0;
        };
        response[0] = command;

        let response_len = match command {
            ID_DAP_EXECUTE_COMMANDS => self.execute_commands(request, response),
            ID_DAP_QUEUE_COMMANDS => self.execute_commands(request, response),
            ID_DAP_INFO => self.dap_info(request, response),
            ID_DAP_HOST_STATUS => {
                response[1] = DAP_OK;
                2
            }
            ID_DAP_CONNECT => self.connect(request, response),
            ID_DAP_DISCONNECT => {
                self.port = Port::Disabled;
                response[1] = DAP_OK;
                2
            }
            ID_DAP_TRANSFER_CONFIGURE => {
                if request.len() >= 6 {
                    self.swj.set_idle_cycles(request[1]);
                    self.wait_retries = u16::from_le_bytes([request[2], request[3]]) as usize;
                    self.match_retries = u16::from_le_bytes([request[4], request[5]]) as usize;
                    response[1] = DAP_OK;
                } else {
                    response[1] = DAP_ERROR;
                }
                2
            }
            ID_DAP_SWJ_CLOCK => {
                if request.len() >= 5 {
                    self.swj.set_clock_hz(u32::from_le_bytes([
                        request[1], request[2], request[3], request[4],
                    ]));
                    response[1] = DAP_OK;
                } else {
                    response[1] = DAP_ERROR;
                }
                2
            }
            ID_DAP_SWJ_PINS => self.swj_pins(request, response),
            ID_DAP_SWJ_SEQUENCE => self.swj_sequence(request, response),
            ID_DAP_SWD_CONFIGURE => {
                if let Some(&config) = request.get(1) {
                    self.swj
                        .configure_swd((config & 0b011) + 1, config & 0b100 != 0);
                    response[1] = DAP_OK;
                } else {
                    response[1] = DAP_ERROR;
                }
                2
            }
            ID_DAP_SWD_SEQUENCE => self.swd_sequence(request, response),
            ID_DAP_JTAG_SEQUENCE => self.jtag_sequence(request, response),
            ID_DAP_JTAG_CONFIGURE => self.jtag_configure(request, response),
            ID_DAP_JTAG_IDCODE => self.jtag_idcode(request, response),
            ID_DAP_SWO_TRANSPORT => self.swo_transport(request, response),
            ID_DAP_SWO_MODE | ID_DAP_SWO_CONTROL => {
                response[1] = DAP_ERROR;
                2
            }
            ID_DAP_SWO_BAUDRATE => {
                response[1..5].copy_from_slice(&0u32.to_le_bytes());
                5
            }
            ID_DAP_SWO_STATUS => {
                response[1] = 0;
                response[2..6].copy_from_slice(&0u32.to_le_bytes());
                6
            }
            ID_DAP_SWO_EXTENDED_STATUS => {
                response[1] = 0;
                response[2..6].copy_from_slice(&0u32.to_le_bytes());
                response[6..10].copy_from_slice(&0u32.to_le_bytes());
                response[10..14].copy_from_slice(&0u32.to_le_bytes());
                14
            }
            ID_DAP_SWO_DATA => {
                response[1] = 0;
                response[2..4].copy_from_slice(&0u16.to_le_bytes());
                4
            }
            ID_DAP_TRANSFER => self.transfer(request, response),
            ID_DAP_TRANSFER_BLOCK => self.transfer_block(request, response),
            ID_DAP_WRITE_ABORT => self.write_abort(request, response),
            ID_DAP_TRANSFER_ABORT => {
                response[1] = DAP_OK;
                2
            }
            ID_DAP_DELAY => {
                if request.len() >= 3 {
                    let us = u16::from_le_bytes([request[1], request[2]]);
                    time::delay_micros(us as u64);
                    response[1] = DAP_OK;
                } else {
                    response[1] = DAP_ERROR;
                }
                2
            }
            ID_DAP_RESET_TARGET => {
                self.swj.reset_target();
                response[1] = DAP_OK;
                response[2] = 1;
                3
            }
            _ => {
                response[1] = DAP_ERROR;
                2
            }
        };

        trace_command(command, request, response, response_len);
        response_len
    }

    fn dap_info(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        let Some(&info) = request.get(1) else {
            response[1] = 0;
            return 2;
        };

        match info {
            0x01..=0x03 => {
                response[1] = 0;
                2
            }
            0x04 => info_str(response, DAP_FW_VERSION),
            0x09 => info_str(response, DAP_PRODUCT_FW_VERSION),
            0xf0 => info_capabilities(response),
            0xfe => info_u8(response, 1),
            0xff => info_u16(response, PACKET_SIZE as u16),
            0xfb | 0xfc | 0xfd => info_u32(response, 0),
            _ => {
                response[1] = 0;
                2
            }
        }
    }

    fn connect(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        let requested = request.get(1).copied().unwrap_or(0);
        self.port = match requested {
            0 | 1 => Port::Swd,
            2 => Port::Jtag,
            _ => Port::Disabled,
        };
        self.swj.connect(self.port);
        response[1] = self.port as u8;
        2
    }

    fn swj_pins(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        if request.len() < 7 {
            response[1] = 0;
            return 2;
        }
        let values = request[1];
        let select = request[2] & !0x20;
        let wait_us = u32::from_le_bytes([request[3], request[4], request[5], request[6]]);
        let mut state = self.swj.set_pins(values, select);

        if wait_us != 0 {
            let wait_us = wait_us.min(3_000_000);
            let start = time::now_ticks();
            let timeout = time::ticks_from_micros(wait_us as u64);
            while ((state ^ values) & select) != 0
                && time::now_ticks().wrapping_sub(start) < timeout
            {
                core::hint::spin_loop();
                state = self.swj.pin_state();
            }
        }

        response[1] = state;
        2
    }

    fn execute_commands(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        if request.len() < 2 {
            response[0] = ID_DAP_EXECUTE_COMMANDS;
            response[1] = 0;
            return 2;
        }

        let requested = request[1] as usize;
        let mut input = 2;
        let mut output = 2;
        let mut executed = 0usize;

        response[0] = ID_DAP_EXECUTE_COMMANDS;

        for _ in 0..requested {
            if input >= request.len() || output >= PACKET_SIZE {
                break;
            }

            let Some(request_len) = command_request_len(&request[input..]) else {
                response[output] = 0xff;
                output += 1;
                input += 1;
                executed += 1;
                continue;
            };
            if input + request_len > request.len() {
                break;
            }

            let mut sub_response = [0u8; PACKET_SIZE];
            let response_len =
                self.process(&request[input..input + request_len], &mut sub_response);
            if output + response_len > PACKET_SIZE {
                break;
            }
            response[output..output + response_len].copy_from_slice(&sub_response[..response_len]);

            input += request_len;
            output += response_len;
            executed += 1;
        }

        response[1] = executed as u8;
        output
    }

    fn swj_sequence(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        let Some(&count_byte) = request.get(1) else {
            response[1] = DAP_ERROR;
            return 2;
        };
        let bit_count = if count_byte == 0 {
            256
        } else {
            count_byte as usize
        };
        let byte_count = bit_count.div_ceil(8);
        if request.len() < 2 + byte_count {
            response[1] = DAP_ERROR;
            return 2;
        }
        self.swj
            .swj_sequence(bit_count, &request[2..2 + byte_count]);
        response[1] = DAP_OK;
        2
    }

    fn swd_sequence(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        if self.port != Port::Swd || request.len() < 2 {
            response[1] = DAP_ERROR;
            return 2;
        }

        response[1] = DAP_OK;

        let sequence_count = request[1] as usize;
        let mut input = 2;
        let mut output = 2;

        for _ in 0..sequence_count {
            let Some(&info) = request.get(input) else {
                response[1] = DAP_ERROR;
                return 2;
            };
            input += 1;

            let bit_count = match info & 0x3f {
                0 => 64,
                n => n as usize,
            };
            let byte_count = bit_count.div_ceil(8);
            let read = info & 0x80 != 0;

            if read {
                if output + byte_count > PACKET_SIZE {
                    response[1] = DAP_ERROR;
                    return 2;
                }
                self.swj
                    .swd_read_sequence(bit_count, &mut response[output..output + byte_count]);
                output += byte_count;
            } else {
                let Some(data) = request.get(input..input + byte_count) else {
                    response[1] = DAP_ERROR;
                    return 2;
                };
                self.swj.swd_write_sequence(bit_count, data);
                input += byte_count;
            }
        }

        output
    }

    fn swo_transport(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        match request.get(1).copied().unwrap_or(0) {
            0 | 1 => response[1] = DAP_OK,
            _ => response[1] = DAP_ERROR,
        }
        2
    }

    fn jtag_configure(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        let Some(&count) = request.get(1) else {
            response[1] = DAP_ERROR;
            return 2;
        };
        let count = count as usize;
        if count > JTAG_DEV_MAX || request.len() < 2 + count {
            response[1] = DAP_ERROR;
            return 2;
        }

        let mut bits = 0u16;
        for n in 0..count {
            let length = request[2 + n];
            if length == 0 {
                response[1] = DAP_ERROR;
                return 2;
            }
            self.jtag_ir_length[n] = length;
            self.jtag_ir_before[n] = bits;
            let Some(next) = bits.checked_add(length as u16) else {
                response[1] = DAP_ERROR;
                return 2;
            };
            bits = next;
        }

        let mut after = bits;
        for n in 0..count {
            after -= self.jtag_ir_length[n] as u16;
            self.jtag_ir_after[n] = after;
        }

        for n in count..JTAG_DEV_MAX {
            self.jtag_ir_length[n] = 0;
            self.jtag_ir_before[n] = 0;
            self.jtag_ir_after[n] = 0;
        }

        self.jtag_device_count = count as u8;
        self.jtag_index = 0;
        response[1] = DAP_OK;
        2
    }

    fn jtag_idcode(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        let Some(&index) = request.get(1) else {
            response[1] = DAP_ERROR;
            return 2;
        };
        if self.port != Port::Jtag || !self.select_jtag_index(index) {
            response[1] = DAP_ERROR;
            return 2;
        }

        self.jtag_set_ir(JTAG_IDCODE);
        let idcode = self.swj.jtag_read_idcode(self.jtag_index as usize);
        response[1] = DAP_OK;
        response[2..6].copy_from_slice(&idcode.to_le_bytes());
        6
    }

    fn jtag_sequence(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        let Some(&sequence_count) = request.get(1) else {
            response[1] = DAP_ERROR;
            return 2;
        };

        let mut input = 2;
        let mut output = 2;

        for _ in 0..sequence_count {
            let Some(&info) = request.get(input) else {
                response[1] = DAP_ERROR;
                return 2;
            };
            input += 1;

            let bit_count = match info & 0x3f {
                0 => 64,
                n => n as usize,
            };
            let byte_count = bit_count.div_ceil(8);
            if request.len() < input + byte_count || output + byte_count > PACKET_SIZE {
                response[1] = DAP_ERROR;
                return 2;
            }

            let tms = info & 0x40 != 0;
            let capture_tdo = info & 0x80 != 0;
            let mut captured = [0u8; 8];
            let captured_len = self.swj.jtag_sequence(
                bit_count,
                tms,
                &request[input..input + byte_count],
                capture_tdo,
                &mut captured,
            );
            input += byte_count;

            if capture_tdo {
                response[output..output + captured_len].copy_from_slice(&captured[..captured_len]);
                output += captured_len;
            }
        }

        response[1] = DAP_OK;
        output
    }

    #[inline(never)]
    #[unsafe(link_section = ".fast")]
    fn transfer(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        if request.len() < 3 {
            response[1] = 0;
            response[2] = DAP_TRANSFER_ERROR;
            return 3;
        }

        if self.port == Port::Jtag {
            return self.jtag_transfer(request, response);
        }

        if self.port != Port::Swd {
            response[1] = 0;
            response[2] = 0;
            return 3;
        }

        let transfer_count = request[2] as usize;
        let mut input = 3;
        let mut output = 3;
        let mut done = 0u8;
        let mut status = 0;
        let mut post_read = false;
        let mut check_write = false;

        for _ in 0..transfer_count {
            let Some(&dap_request) = request.get(input) else {
                status = DAP_TRANSFER_ERROR;
                break;
            };
            input += 1;

            if dap_request & DAP_TRANSFER_RNW != 0 {
                if post_read {
                    let result = if (dap_request & (DAP_TRANSFER_APNDP | DAP_TRANSFER_MATCH_VALUE))
                        == DAP_TRANSFER_APNDP
                    {
                        self.swd_transfer_with_retry(dap_request, 0)
                    } else {
                        post_read = false;
                        self.swd_transfer_with_retry(DP_RDBUFF_READ, 0)
                    };
                    status = transfer_status(result.status);
                    if status != DAP_TRANSFER_OK {
                        break;
                    }
                    if output + 4 > PACKET_SIZE {
                        status = DAP_TRANSFER_ERROR;
                        break;
                    }
                    response[output..output + 4].copy_from_slice(&result.data.to_le_bytes());
                    trace_event(0x4800_0005, done as u32, dap_request as u32, result.data);
                    output += 4;
                }

                if dap_request & DAP_TRANSFER_MATCH_VALUE != 0 {
                    let Some(bytes) = request.get(input..input + 4) else {
                        status = DAP_TRANSFER_ERROR;
                        break;
                    };
                    input += 4;
                    let target = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

                    if dap_request & DAP_TRANSFER_APNDP != 0 {
                        let result = self.swd_transfer_with_retry(dap_request, 0);
                        status = transfer_status(result.status);
                        if status != DAP_TRANSFER_OK {
                            break;
                        }
                    }

                    let mut match_retry = self.match_retries;
                    loop {
                        let result = self.swd_transfer_with_retry(dap_request, 0);
                        status = transfer_status(result.status);
                        trace_event(0x4801_0005, done as u32, dap_request as u32, result.data);
                        if status != DAP_TRANSFER_OK {
                            break;
                        }
                        if (result.data & self.match_mask) == target {
                            break;
                        }
                        if match_retry == 0 {
                            status = DAP_TRANSFER_MISMATCH;
                            break;
                        }
                        match_retry -= 1;
                    }
                    if status != DAP_TRANSFER_OK {
                        break;
                    }
                    post_read = false;
                } else if dap_request & DAP_TRANSFER_APNDP != 0 {
                    if !post_read {
                        let result = self.swd_transfer_with_retry(dap_request, 0);
                        status = transfer_status(result.status);
                        if status != DAP_TRANSFER_OK {
                            break;
                        }
                        post_read = true;
                    }
                } else {
                    let result = self.swd_transfer_with_retry(dap_request, 0);
                    status = transfer_status(result.status);
                    if status != DAP_TRANSFER_OK {
                        break;
                    }
                    if output + 4 > PACKET_SIZE {
                        status = DAP_TRANSFER_ERROR;
                        break;
                    }
                    response[output..output + 4].copy_from_slice(&result.data.to_le_bytes());
                    trace_event(0x4802_0005, done as u32, dap_request as u32, result.data);
                    output += 4;
                }
                check_write = false;
            } else {
                if post_read {
                    let result = self.swd_transfer_with_retry(DP_RDBUFF_READ, 0);
                    status = transfer_status(result.status);
                    if status != DAP_TRANSFER_OK {
                        break;
                    }
                    if output + 4 > PACKET_SIZE {
                        status = DAP_TRANSFER_ERROR;
                        break;
                    }
                    response[output..output + 4].copy_from_slice(&result.data.to_le_bytes());
                    trace_event(0x4810_0005, done as u32, DP_RDBUFF_READ as u32, result.data);
                    output += 4;
                    post_read = false;
                }

                let Some(bytes) = request.get(input..input + 4) else {
                    status = DAP_TRANSFER_ERROR;
                    break;
                };
                input += 4;
                let write_data = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                trace_event(0x4700_0005, done as u32, dap_request as u32, write_data);

                if dap_request & DAP_TRANSFER_MATCH_MASK != 0 {
                    self.match_mask = write_data;
                    status = DAP_TRANSFER_OK;
                    check_write = false;
                } else {
                    let result = self.swd_transfer_with_retry(dap_request, write_data);
                    status = transfer_status(result.status);
                    if status != DAP_TRANSFER_OK {
                        break;
                    }
                    check_write = true;
                };
            }

            done = done.wrapping_add(1);
        }

        if status == DAP_TRANSFER_OK {
            if post_read {
                let result = self.swd_transfer_with_retry(DP_RDBUFF_READ, 0);
                status = transfer_status(result.status);
                if status == DAP_TRANSFER_OK && output + 4 <= PACKET_SIZE {
                    response[output..output + 4].copy_from_slice(&result.data.to_le_bytes());
                    trace_event(0x4811_0005, done as u32, DP_RDBUFF_READ as u32, result.data);
                    output += 4;
                } else if status == DAP_TRANSFER_OK {
                    status = DAP_TRANSFER_ERROR;
                }
            } else if check_write && CHECK_POSTED_WRITES {
                let result = self.swd_transfer_with_retry(DP_RDBUFF_READ, 0);
                status = transfer_status(result.status);
                trace_event(0x4820_0005, done as u32, status as u32, result.data);
            }
        }

        response[1] = done;
        response[2] = status;
        output
    }

    #[inline(never)]
    #[unsafe(link_section = ".fast")]
    fn transfer_block(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        if request.len() < 5 {
            response[1] = 0;
            response[2] = 0;
            response[3] = DAP_TRANSFER_ERROR;
            return 4;
        }

        if self.port == Port::Jtag {
            return self.jtag_transfer_block(request, response);
        }

        if self.port != Port::Swd {
            response[1] = 0;
            response[2] = 0;
            response[3] = 0;
            return 4;
        }

        let transfer_count = u16::from_le_bytes([request[2], request[3]]) as usize;
        let dap_request = request[4];
        let is_read = dap_request & DAP_TRANSFER_RNW != 0;
        let input = 5;
        let mut output = 4;
        let mut done = 0usize;
        let mut status = 0;

        if dap_request & DAP_TRANSFER_RNW == 0 {
            trace_event(
                0x4600_0006,
                transfer_count as u32,
                dap_request as u32,
                pack4(request.get(5..9).unwrap_or(&[])),
            );
            trace_event(
                0x4610_0006,
                transfer_count as u32,
                dap_request as u32,
                pack4(request.get(9..13).unwrap_or(&[])),
            );
        }

        if transfer_count == 0 {
        } else if dap_request & (DAP_TRANSFER_MATCH_VALUE | DAP_TRANSFER_MATCH_MASK) != 0 {
            status = DAP_TRANSFER_ERROR;
        } else if is_read {
            let mut request_value = dap_request;
            if request_value & DAP_TRANSFER_APNDP != 0 {
                let result = self.swd_transfer_with_retry(request_value, 0);
                status = transfer_status(result.status);
                if status != DAP_TRANSFER_OK {
                    response[1..3].copy_from_slice(&(done as u16).to_le_bytes());
                    response[3] = status;
                    return output;
                }
            }

            while done < transfer_count {
                if done + 1 == transfer_count && dap_request & DAP_TRANSFER_APNDP != 0 {
                    request_value = DP_RDBUFF_READ;
                }

                let result = self.swd_transfer_with_retry(request_value, 0);
                status = transfer_status(result.status);
                if status != DAP_TRANSFER_OK {
                    break;
                }
                if output + 4 > PACKET_SIZE {
                    status = DAP_TRANSFER_ERROR;
                    break;
                }
                response[output..output + 4].copy_from_slice(&result.data.to_le_bytes());
                trace_event(0x4800_0006, done as u32, request_value as u32, result.data);
                output += 4;
                done += 1;
            }
        } else {
            let byte_count = transfer_count.saturating_mul(4);
            if let Some(bytes) = request.get(input..input + byte_count) {
                let result =
                    self.swj
                        .swd_write_block(dap_request, bytes, transfer_count, self.wait_retries);
                done = result.done;
                status = transfer_status(result.status);
            } else {
                status = DAP_TRANSFER_ERROR;
            }

            if status == DAP_TRANSFER_OK && CHECK_POSTED_WRITES {
                let result = self.swd_transfer_with_retry(DP_RDBUFF_READ, 0);
                status = transfer_status(result.status);
                trace_event(0x4820_0006, done as u32, status as u32, result.data);
            }
        }

        let done = done as u16;
        response[1..3].copy_from_slice(&done.to_le_bytes());
        response[3] = status;
        output
    }

    fn write_abort(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        if request.len() < 6 {
            response[1] = DAP_TRANSFER_ERROR;
            return 2;
        }

        if self.port == Port::Jtag {
            let index = request[1];
            if !self.select_jtag_index(index) {
                response[1] = DAP_ERROR;
                return 2;
            }

            let data = u32::from_le_bytes([request[2], request[3], request[4], request[5]]);
            self.jtag_set_ir(JTAG_ABORT);
            self.swj.jtag_write_abort(
                self.jtag_index as usize,
                self.jtag_device_count as usize,
                data,
            );
            response[1] = DAP_OK;
            return 2;
        }

        if self.port != Port::Swd {
            response[1] = DAP_TRANSFER_ERROR;
            return 2;
        }

        let data = u32::from_le_bytes([request[2], request[3], request[4], request[5]]);
        let _ = self.swd_transfer_with_retry(0x00, data);
        response[1] = DAP_OK;
        2
    }

    #[inline(always)]
    #[unsafe(link_section = ".fast")]
    fn swd_transfer_with_retry(
        &mut self,
        dap_request: u8,
        write_data: u32,
    ) -> crate::swj::TransferResult {
        let mut result = self.swj.swd_transfer(dap_request, write_data);
        let mut retry = self.wait_retries;
        while result.status == TransferStatus::Wait && retry != 0 {
            retry -= 1;
            result = self.swj.swd_transfer(dap_request, write_data);
        }
        result
    }

    fn jtag_transfer(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        if !self.select_jtag_index(request[1]) {
            response[1] = 0;
            response[2] = 0;
            return 3;
        }

        let transfer_count = request[2] as usize;
        let mut input = 3;
        let mut output = 3;
        let mut done = 0u8;
        let mut status = 0;
        let mut current_ir = 0u8;
        let mut post_read = false;
        let mut touched_bus = false;

        for _ in 0..transfer_count {
            let Some(&dap_request) = request.get(input) else {
                status = DAP_TRANSFER_ERROR;
                break;
            };
            input += 1;

            let request_ir = jtag_ir_for_request(dap_request);
            if dap_request & DAP_TRANSFER_RNW != 0 {
                if post_read {
                    let result = if current_ir == request_ir
                        && dap_request & DAP_TRANSFER_MATCH_VALUE == 0
                    {
                        self.jtag_transfer_with_ir(&mut current_ir, request_ir, dap_request, 0)
                    } else {
                        post_read = false;
                        self.jtag_transfer_with_ir(&mut current_ir, JTAG_DPACC, DP_RDBUFF_READ, 0)
                    };
                    touched_bus = true;
                    status = transfer_status(result.status);
                    if status != DAP_TRANSFER_OK {
                        break;
                    }
                    if output + 4 > PACKET_SIZE {
                        status = DAP_TRANSFER_ERROR;
                        break;
                    }
                    response[output..output + 4].copy_from_slice(&result.data.to_le_bytes());
                    output += 4;
                }

                if dap_request & DAP_TRANSFER_MATCH_VALUE != 0 {
                    let Some(bytes) = request.get(input..input + 4) else {
                        status = DAP_TRANSFER_ERROR;
                        break;
                    };
                    input += 4;
                    let target = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

                    let result =
                        self.jtag_transfer_with_ir(&mut current_ir, request_ir, dap_request, 0);
                    touched_bus = true;
                    status = transfer_status(result.status);
                    if status != DAP_TRANSFER_OK {
                        break;
                    }

                    let mut match_retry = self.match_retries;
                    loop {
                        let result =
                            self.jtag_transfer_with_ir(&mut current_ir, request_ir, dap_request, 0);
                        touched_bus = true;
                        status = transfer_status(result.status);
                        if status != DAP_TRANSFER_OK {
                            break;
                        }
                        if (result.data & self.match_mask) == target {
                            break;
                        }
                        if match_retry == 0 {
                            status = DAP_TRANSFER_MISMATCH;
                            break;
                        }
                        match_retry -= 1;
                    }
                    if status != DAP_TRANSFER_OK {
                        break;
                    }
                    post_read = false;
                } else if !post_read {
                    let result =
                        self.jtag_transfer_with_ir(&mut current_ir, request_ir, dap_request, 0);
                    touched_bus = true;
                    status = transfer_status(result.status);
                    if status != DAP_TRANSFER_OK {
                        break;
                    }
                    post_read = true;
                }
            } else {
                if post_read {
                    let result =
                        self.jtag_transfer_with_ir(&mut current_ir, JTAG_DPACC, DP_RDBUFF_READ, 0);
                    touched_bus = true;
                    status = transfer_status(result.status);
                    if status != DAP_TRANSFER_OK {
                        break;
                    }
                    if output + 4 > PACKET_SIZE {
                        status = DAP_TRANSFER_ERROR;
                        break;
                    }
                    response[output..output + 4].copy_from_slice(&result.data.to_le_bytes());
                    output += 4;
                    post_read = false;
                }

                let Some(bytes) = request.get(input..input + 4) else {
                    status = DAP_TRANSFER_ERROR;
                    break;
                };
                input += 4;
                let write_data = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

                if dap_request & DAP_TRANSFER_MATCH_MASK != 0 {
                    self.match_mask = write_data;
                    status = DAP_TRANSFER_OK;
                } else {
                    let result = self.jtag_transfer_with_ir(
                        &mut current_ir,
                        request_ir,
                        dap_request,
                        write_data,
                    );
                    touched_bus = true;
                    status = transfer_status(result.status);
                    if status != DAP_TRANSFER_OK {
                        break;
                    }
                }
            }

            done = done.wrapping_add(1);
        }

        if status == DAP_TRANSFER_OK && touched_bus {
            let result = self.jtag_transfer_with_ir(&mut current_ir, JTAG_DPACC, DP_RDBUFF_READ, 0);
            status = transfer_status(result.status);
            if status == DAP_TRANSFER_OK && post_read {
                if output + 4 <= PACKET_SIZE {
                    response[output..output + 4].copy_from_slice(&result.data.to_le_bytes());
                    output += 4;
                } else {
                    status = DAP_TRANSFER_ERROR;
                }
            }
        }

        response[1] = done;
        response[2] = status;
        output
    }

    fn jtag_transfer_block(&mut self, request: &[u8], response: &mut [u8; PACKET_SIZE]) -> usize {
        if !self.select_jtag_index(request[1]) {
            response[1] = 0;
            response[2] = 0;
            response[3] = 0;
            return 4;
        }

        let transfer_count = u16::from_le_bytes([request[2], request[3]]) as usize;
        let dap_request = request[4];
        let is_read = dap_request & DAP_TRANSFER_RNW != 0;
        let input = 5;
        let mut output = 4;
        let mut done = 0usize;
        let mut status = 0;
        let mut current_ir = 0u8;

        if transfer_count == 0 {
        } else if dap_request & (DAP_TRANSFER_MATCH_VALUE | DAP_TRANSFER_MATCH_MASK) != 0 {
            status = DAP_TRANSFER_ERROR;
        } else if is_read {
            let request_ir = jtag_ir_for_request(dap_request);
            let result = self.jtag_transfer_with_ir(&mut current_ir, request_ir, dap_request, 0);
            status = transfer_status(result.status);

            while status == DAP_TRANSFER_OK && done < transfer_count {
                let (ir, request_value) = if done + 1 == transfer_count {
                    (JTAG_DPACC, DP_RDBUFF_READ)
                } else {
                    (request_ir, dap_request)
                };
                let result = self.jtag_transfer_with_ir(&mut current_ir, ir, request_value, 0);
                status = transfer_status(result.status);
                if status != DAP_TRANSFER_OK {
                    break;
                }
                if output + 4 > PACKET_SIZE {
                    status = DAP_TRANSFER_ERROR;
                    break;
                }
                response[output..output + 4].copy_from_slice(&result.data.to_le_bytes());
                output += 4;
                done += 1;
            }
        } else {
            let request_ir = jtag_ir_for_request(dap_request);
            let byte_count = transfer_count.saturating_mul(4);
            if let Some(bytes) = request.get(input..input + byte_count) {
                self.jtag_set_ir(request_ir);
                current_ir = request_ir;
                let result = self.swj.jtag_write_block(
                    self.jtag_index as usize,
                    self.jtag_device_count as usize,
                    dap_request,
                    bytes,
                    transfer_count,
                    self.wait_retries,
                );
                done = result.done;
                status = transfer_status(result.status);
            } else {
                status = DAP_TRANSFER_ERROR;
            }

            if status == DAP_TRANSFER_OK {
                let result =
                    self.jtag_transfer_with_ir(&mut current_ir, JTAG_DPACC, DP_RDBUFF_READ, 0);
                status = transfer_status(result.status);
            }
        }

        response[1..3].copy_from_slice(&(done as u16).to_le_bytes());
        response[3] = status;
        output
    }

    fn select_jtag_index(&mut self, index: u8) -> bool {
        if index < self.jtag_device_count {
            self.jtag_index = index;
            true
        } else {
            false
        }
    }

    fn jtag_set_ir(&mut self, ir: u8) {
        let index = self.jtag_index as usize;
        self.swj.jtag_ir(
            ir as u32,
            self.jtag_ir_length[index],
            self.jtag_ir_before[index],
            self.jtag_ir_after[index],
        );
    }

    fn jtag_transfer_with_ir(
        &mut self,
        current_ir: &mut u8,
        ir: u8,
        dap_request: u8,
        write_data: u32,
    ) -> crate::swj::TransferResult {
        if *current_ir != ir {
            self.jtag_set_ir(ir);
            *current_ir = ir;
        }

        let mut result = self.swj.jtag_transfer(
            self.jtag_index as usize,
            self.jtag_device_count as usize,
            dap_request,
            write_data,
        );
        let mut retry = self.wait_retries;
        while result.status == TransferStatus::Wait && retry != 0 {
            retry -= 1;
            result = self.swj.jtag_transfer(
                self.jtag_index as usize,
                self.jtag_device_count as usize,
                dap_request,
                write_data,
            );
        }
        result
    }
}

fn info_str(response: &mut [u8; PACKET_SIZE], value: &str) -> usize {
    let bytes = value.as_bytes();
    let len = bytes.len().min(PACKET_SIZE - 2);
    response[1] = len as u8;
    response[2..2 + len].copy_from_slice(&bytes[..len]);
    2 + len
}

fn info_u8(response: &mut [u8; PACKET_SIZE], value: u8) -> usize {
    response[1] = 1;
    response[2] = value;
    3
}

fn info_u16(response: &mut [u8; PACKET_SIZE], value: u16) -> usize {
    response[1] = 2;
    response[2..4].copy_from_slice(&value.to_le_bytes());
    4
}

fn info_u32(response: &mut [u8; PACKET_SIZE], value: u32) -> usize {
    response[1] = 4;
    response[2..6].copy_from_slice(&value.to_le_bytes());
    6
}

fn info_capabilities(response: &mut [u8; PACKET_SIZE]) -> usize {
    response[1] = 1;
    response[2] = 0x03;
    3
}

fn jtag_ir_for_request(request: u8) -> u8 {
    if request & DAP_TRANSFER_APNDP != 0 {
        JTAG_APACC
    } else {
        JTAG_DPACC
    }
}

fn transfer_status(status: TransferStatus) -> u8 {
    match status {
        TransferStatus::Ok => DAP_TRANSFER_OK,
        TransferStatus::Wait => DAP_TRANSFER_WAIT,
        TransferStatus::Fault => DAP_TRANSFER_FAULT,
        TransferStatus::NoAck => DAP_TRANSFER_NO_ACK,
        TransferStatus::ProtocolError | TransferStatus::ParityError => DAP_TRANSFER_ERROR,
    }
}

fn command_request_len(request: &[u8]) -> Option<usize> {
    let command = *request.first()?;
    match command {
        ID_DAP_INFO => Some(2),
        ID_DAP_HOST_STATUS => Some(3),
        ID_DAP_CONNECT => Some(2),
        ID_DAP_DISCONNECT | ID_DAP_TRANSFER_ABORT | ID_DAP_RESET_TARGET => Some(1),
        ID_DAP_TRANSFER_CONFIGURE => Some(6),
        ID_DAP_SWJ_CLOCK => Some(5),
        ID_DAP_SWJ_PINS => Some(7),
        ID_DAP_SWD_CONFIGURE => Some(2),
        ID_DAP_DELAY => Some(3),
        ID_DAP_WRITE_ABORT => Some(6),
        ID_DAP_SWJ_SEQUENCE => {
            let count = *request.get(1)?;
            Some(2 + bit_count_from_byte(count).div_ceil(8))
        }
        ID_DAP_SWD_SEQUENCE => swd_sequence_request_len(request),
        ID_DAP_JTAG_CONFIGURE => {
            let count = *request.get(1)? as usize;
            Some(2 + count)
        }
        ID_DAP_JTAG_SEQUENCE => jtag_sequence_request_len(request),
        ID_DAP_JTAG_IDCODE => Some(2),
        ID_DAP_TRANSFER => transfer_request_len(request),
        ID_DAP_TRANSFER_BLOCK => transfer_block_request_len(request),
        ID_DAP_SWO_TRANSPORT | ID_DAP_SWO_MODE | ID_DAP_SWO_CONTROL => Some(2),
        ID_DAP_SWO_BAUDRATE => Some(5),
        ID_DAP_SWO_STATUS => Some(1),
        ID_DAP_SWO_EXTENDED_STATUS => Some(2),
        ID_DAP_SWO_DATA => Some(3),
        ID_DAP_QUEUE_COMMANDS | ID_DAP_EXECUTE_COMMANDS => None,
        _ => Some(1),
    }
}

fn transfer_request_len(request: &[u8]) -> Option<usize> {
    let transfer_count = *request.get(2)? as usize;
    let mut input = 3;
    for _ in 0..transfer_count {
        let dap_request = *request.get(input)?;
        input += 1;
        if dap_request & DAP_TRANSFER_RNW != 0 {
            if dap_request & DAP_TRANSFER_MATCH_VALUE != 0 {
                input += 4;
            }
        } else {
            input += 4;
        }
    }
    Some(input)
}

fn transfer_block_request_len(request: &[u8]) -> Option<usize> {
    let count = u16::from_le_bytes([*request.get(2)?, *request.get(3)?]) as usize;
    let dap_request = *request.get(4)?;
    if dap_request & DAP_TRANSFER_RNW != 0 {
        Some(5)
    } else {
        Some(5 + count * 4)
    }
}

fn swd_sequence_request_len(request: &[u8]) -> Option<usize> {
    let sequence_count = *request.get(1)? as usize;
    let mut input = 2;
    for _ in 0..sequence_count {
        let info = *request.get(input)?;
        input += 1;
        let byte_count = bit_count_from_swd_sequence_info(info).div_ceil(8);
        if info & 0x80 == 0 {
            input += byte_count;
        }
    }
    Some(input)
}

fn jtag_sequence_request_len(request: &[u8]) -> Option<usize> {
    let sequence_count = *request.get(1)? as usize;
    let mut input = 2;
    for _ in 0..sequence_count {
        let info = *request.get(input)?;
        input += 1;
        input += bit_count_from_swd_sequence_info(info).div_ceil(8);
    }
    Some(input)
}

fn bit_count_from_byte(count: u8) -> usize {
    if count == 0 { 256 } else { count as usize }
}

fn bit_count_from_swd_sequence_info(info: u8) -> usize {
    match info & 0x3f {
        0 => 64,
        n => n as usize,
    }
}

fn trace_command(command: u8, request: &[u8], response: &[u8], response_len: usize) {
    if !ENABLE_DAP_TRACE {
        return;
    }

    let request_word = pack4(request);
    let response_word = pack4(response);
    trace_event(
        0x4400_0000 | command as u32,
        request.len() as u32,
        response_len as u32,
        request_word,
    );
    trace_event(
        0x4500_0000 | command as u32,
        response.get(1).copied().unwrap_or(0) as u32,
        response.get(2).copied().unwrap_or(0) as u32,
        response_word,
    );
}

fn pack4(bytes: &[u8]) -> u32 {
    let b0 = bytes.first().copied().unwrap_or(0) as u32;
    let b1 = bytes.get(1).copied().unwrap_or(0) as u32;
    let b2 = bytes.get(2).copied().unwrap_or(0) as u32;
    let b3 = bytes.get(3).copied().unwrap_or(0) as u32;
    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}

fn trace_event(tag: u32, a: u32, b: u32, c: u32) {
    if !ENABLE_DAP_TRACE {
        return;
    }

    unsafe {
        let index = core::ptr::read_volatile(&raw const DAP_TRACE_INDEX) as usize;
        let base = (&raw mut YBLINK_DAP_TRACE) as *mut [u32; 1024] as *mut u32;
        core::ptr::write_volatile(base.add(index & 1023), tag);
        core::ptr::write_volatile(base.add((index + 1) & 1023), a);
        core::ptr::write_volatile(base.add((index + 2) & 1023), b);
        core::ptr::write_volatile(base.add((index + 3) & 1023), c);
        core::ptr::write_volatile(&raw mut DAP_TRACE_INDEX, index.wrapping_add(4) as u32);
    }
}
