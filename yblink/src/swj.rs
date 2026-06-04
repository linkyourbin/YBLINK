use hpm5301_hal::time;

#[unsafe(no_mangle)]
#[used]
static mut YBLINK_SWD_TRACE: [u32; 16] = [0; 16];
const ENABLE_SWD_TRACE: bool = false;

pub trait ProbeIo {
    fn set_clock_hz(&mut self, hz: u32);
    fn swdio_output(&mut self);
    fn swdio_input(&mut self);
    fn set_swdio_tms(&mut self, high: bool);
    fn set_tdi(&mut self, high: bool);
    fn set_swclk_tck(&mut self, high: bool);
    fn set_reset(&mut self, high: bool);
    fn swclk_cycle(&mut self);
    fn swclk_sample_swdio(&mut self) -> bool;
    fn swd_write_bit_cycle(&mut self, high: bool);
    fn swd_write_bits(&mut self, bits: u32, count: usize);
    fn swd_write_data_bits(&mut self, bits: u32, count: usize);
    fn swclk_sample_swdio_bits(&mut self, count: usize) -> u32;
    fn tck_cycle(&mut self);
    fn tck_sample_tdo(&mut self) -> bool;
    fn current_pin_state(&self) -> u8;

    #[inline(always)]
    fn jtag_cycle_tdi(&mut self, high: bool) -> bool {
        let _ = high;
        false
    }

    #[inline(always)]
    fn jtag_cycle_tdio(&mut self, high: bool) -> Option<bool> {
        let _ = high;
        None
    }

    #[inline(always)]
    fn swj_sequence(&mut self, bit_count: usize, data: &[u8]) -> bool {
        let _ = (bit_count, data);
        false
    }

    #[inline(always)]
    fn swd_write_sequence(&mut self, bit_count: usize, data: &[u8]) -> bool {
        let _ = (bit_count, data);
        false
    }

    #[inline(always)]
    fn swd_read_sequence(&mut self, bit_count: usize, out: &mut [u8]) -> bool {
        let _ = (bit_count, out);
        false
    }

    #[inline(always)]
    fn swd_transfer(
        &mut self,
        request: u8,
        write_data: u32,
        turnaround: u8,
        idle_cycles: u8,
        data_phase_on_wait_fault: bool,
    ) -> Option<TransferResult> {
        let _ = (
            request,
            write_data,
            turnaround,
            idle_cycles,
            data_phase_on_wait_fault,
        );
        None
    }

    #[inline(always)]
    fn swd_transfer_precomputed(
        &mut self,
        swd_request: u8,
        write_data: u32,
        turnaround: u8,
        idle_cycles: u8,
        data_phase_on_wait_fault: bool,
    ) -> Option<TransferResult> {
        let _ = (
            swd_request,
            write_data,
            turnaround,
            idle_cycles,
            data_phase_on_wait_fault,
        );
        None
    }

    #[inline(always)]
    fn swd_write_block(
        &mut self,
        request: u8,
        data: &[u8],
        count: usize,
        wait_retries: usize,
        turnaround: u8,
        idle_cycles: u8,
        data_phase_on_wait_fault: bool,
    ) -> Option<WriteBlockResult> {
        let _ = (
            request,
            data,
            count,
            wait_retries,
            turnaround,
            idle_cycles,
            data_phase_on_wait_fault,
        );
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port {
    Disabled = 0,
    Swd = 1,
    #[allow(dead_code)]
    Jtag = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferStatus {
    Ok,
    Wait,
    Fault,
    NoAck,
    ProtocolError,
    ParityError,
}

pub struct TransferResult {
    pub status: TransferStatus,
    pub data: u32,
}

pub struct WriteBlockResult {
    pub status: TransferStatus,
    pub done: usize,
}

pub struct Swj<P: ProbeIo> {
    pins: P,
    idle_cycles: u8,
    turnaround_cycles: u8,
    data_phase_on_wait_fault: bool,
}

impl<P: ProbeIo> Swj<P> {
    pub fn new(pins: P) -> Self {
        Self {
            pins,
            idle_cycles: 0,
            turnaround_cycles: 1,
            data_phase_on_wait_fault: false,
        }
    }

    pub fn set_clock_hz(&mut self, hz: u32) {
        self.pins.set_clock_hz(hz);
    }

    pub fn set_idle_cycles(&mut self, cycles: u8) {
        self.idle_cycles = cycles;
    }

    pub fn configure_swd(&mut self, turnaround_cycles: u8, data_phase_on_wait_fault: bool) {
        self.turnaround_cycles = turnaround_cycles.clamp(1, 4);
        self.data_phase_on_wait_fault = data_phase_on_wait_fault;
    }

    pub fn connect(&mut self, port: Port) {
        match port {
            Port::Swd => self.connect_swd(),
            Port::Jtag => self.connect_jtag(),
            Port::Disabled => {}
        }
    }

    pub fn swj_sequence(&mut self, bit_count: usize, data: &[u8]) {
        if self.pins.swj_sequence(bit_count, data) {
            return;
        }
        self.swd_write_sequence(bit_count, data);
    }

    pub fn swd_write_sequence(&mut self, bit_count: usize, data: &[u8]) {
        if self.pins.swd_write_sequence(bit_count, data) {
            return;
        }
        self.pins.swdio_output();
        for bit_index in 0..bit_count {
            let bit = (data[bit_index / 8] >> (bit_index % 8)) & 1 != 0;
            self.pins.swd_write_bit_cycle(bit);
        }
    }

    pub fn swd_read_sequence(&mut self, bit_count: usize, out: &mut [u8]) {
        out.fill(0);
        if self.pins.swd_read_sequence(bit_count, out) {
            return;
        }
        self.pins.swdio_input();
        for bit_index in 0..bit_count {
            if self.pins.swclk_sample_swdio() {
                out[bit_index / 8] |= 1 << (bit_index % 8);
            }
        }
        self.pins.swdio_output();
        self.pins.set_swdio_tms(true);
    }

    pub fn jtag_sequence(
        &mut self,
        bit_count: usize,
        tms: bool,
        tdi: &[u8],
        capture_tdo: bool,
        out: &mut [u8],
    ) -> usize {
        self.pins.swdio_output();
        self.pins.set_swdio_tms(tms);
        let mut captured_bits = 0;

        for bit_index in 0..bit_count {
            let bit = (tdi[bit_index / 8] >> (bit_index % 8)) & 1 != 0;
            self.pins.set_tdi(bit);
            let tdo = self.pins.tck_sample_tdo();
            if capture_tdo {
                if tdo {
                    out[captured_bits / 8] |= 1 << (captured_bits % 8);
                }
                captured_bits += 1;
            }
        }

        captured_bits.div_ceil(8)
    }

    pub fn jtag_ir(&mut self, mut ir: u32, ir_length: u8, ir_before: u16, ir_after: u16) {
        if ir_length == 0 {
            return;
        }

        self.pins.swdio_output();
        self.pins.set_swdio_tms(true);
        self.jtag_cycle(); // Select-DR-Scan
        self.jtag_cycle(); // Select-IR-Scan
        self.pins.set_swdio_tms(false);
        self.jtag_cycle(); // Capture-IR
        self.jtag_cycle(); // Shift-IR

        self.pins.set_tdi(true);
        for _ in 0..ir_before {
            self.jtag_cycle();
        }

        for _ in 1..ir_length {
            self.jtag_cycle_tdi(ir & 1 != 0);
            ir >>= 1;
        }

        if ir_after != 0 {
            self.jtag_cycle_tdi(ir & 1 != 0);
            self.pins.set_tdi(true);
            for _ in 1..ir_after {
                self.jtag_cycle();
            }
            self.pins.set_swdio_tms(true);
            self.jtag_cycle(); // Bypass and Exit1-IR
        } else {
            self.pins.set_swdio_tms(true);
            self.jtag_cycle_tdi(ir & 1 != 0); // Last IR bit and Exit1-IR
        }

        self.jtag_cycle(); // Update-IR
        self.pins.set_swdio_tms(false);
        self.jtag_cycle(); // Run-Test/Idle
        self.pins.set_tdi(true);
    }

    pub fn jtag_read_idcode(&mut self, index: usize) -> u32 {
        self.pins.swdio_output();
        self.pins.set_swdio_tms(true);
        self.jtag_cycle(); // Select-DR-Scan
        self.pins.set_swdio_tms(false);
        self.jtag_cycle(); // Capture-DR
        self.jtag_cycle(); // Shift-DR

        for _ in 0..index {
            self.jtag_cycle();
        }

        let mut value = 0u32;
        for _ in 0..31 {
            let bit = self.jtag_cycle_tdo();
            value |= (bit as u32) << 31;
            value >>= 1;
        }
        self.pins.set_swdio_tms(true);
        let bit = self.jtag_cycle_tdo();
        value |= (bit as u32) << 31;

        self.jtag_cycle(); // Update-DR
        self.pins.set_swdio_tms(false);
        self.jtag_cycle(); // Run-Test/Idle
        value
    }

    #[inline(never)]
    #[unsafe(link_section = ".fast")]
    pub fn jtag_transfer(
        &mut self,
        index: usize,
        device_count: usize,
        request: u8,
        write_data: u32,
    ) -> TransferResult {
        let mut ack;

        self.pins.swdio_output();
        self.pins.set_swdio_tms(true);
        self.jtag_cycle(); // Select-DR-Scan
        self.pins.set_swdio_tms(false);
        self.jtag_cycle(); // Capture-DR
        self.jtag_cycle(); // Shift-DR

        for _ in 0..index {
            self.jtag_cycle();
        }

        let bit = self.jtag_cycle_tdio(request & 0x02 != 0);
        ack = (bit as u8) << 1;
        let bit = self.jtag_cycle_tdio(request & 0x04 != 0);
        ack |= bit as u8;
        let bit = self.jtag_cycle_tdio(request & 0x08 != 0);
        ack |= (bit as u8) << 2;

        let status = jtag_ack_status(ack);
        if status != TransferStatus::Ok {
            self.pins.set_swdio_tms(true);
            self.jtag_cycle(); // Exit1-DR
            self.jtag_transfer_exit();
            return TransferResult { status, data: 0 };
        }

        let read = request & 0x02 != 0;
        let mut data = 0u32;
        let after = device_count.saturating_sub(index + 1);
        if read {
            for _ in 0..31 {
                let bit = self.jtag_cycle_tdo();
                data |= (bit as u32) << 31;
                data >>= 1;
            }
            if after != 0 {
                let bit = self.jtag_cycle_tdo();
                for _ in 1..after {
                    self.jtag_cycle();
                }
                self.pins.set_swdio_tms(true);
                self.jtag_cycle(); // Bypass and Exit1-DR
                data |= (bit as u32) << 31;
            } else {
                self.pins.set_swdio_tms(true);
                let bit = self.jtag_cycle_tdo();
                data |= (bit as u32) << 31;
            }
        } else {
            let mut value = write_data;
            for _ in 0..31 {
                self.jtag_cycle_tdi(value & 1 != 0);
                value >>= 1;
            }
            if after != 0 {
                self.jtag_cycle_tdi(value & 1 != 0);
                for _ in 1..after {
                    self.jtag_cycle();
                }
                self.pins.set_swdio_tms(true);
                self.jtag_cycle(); // Bypass and Exit1-DR
            } else {
                self.pins.set_swdio_tms(true);
                self.jtag_cycle_tdi(value & 1 != 0);
            }
        }

        self.jtag_transfer_exit();
        TransferResult {
            status: TransferStatus::Ok,
            data,
        }
    }

    #[inline(never)]
    #[unsafe(link_section = ".fast")]
    pub fn jtag_write_block(
        &mut self,
        index: usize,
        device_count: usize,
        request: u8,
        data: &[u8],
        count: usize,
        wait_retries: usize,
    ) -> WriteBlockResult {
        let mut done = 0usize;
        let mut input = 0usize;

        while done < count {
            let write_data = u32::from_le_bytes([
                data[input],
                data[input + 1],
                data[input + 2],
                data[input + 3],
            ]);

            let mut retry = wait_retries;
            loop {
                let status = self.jtag_write_transfer(index, device_count, request, write_data);
                if status != TransferStatus::Wait || retry == 0 {
                    if status != TransferStatus::Ok {
                        return WriteBlockResult { status, done };
                    }
                    break;
                }
                retry -= 1;
            }

            input += 4;
            done += 1;
        }

        WriteBlockResult {
            status: TransferStatus::Ok,
            done,
        }
    }

    #[inline(always)]
    fn jtag_write_transfer(
        &mut self,
        index: usize,
        device_count: usize,
        request: u8,
        mut write_data: u32,
    ) -> TransferStatus {
        let mut ack;

        self.pins.swdio_output();
        self.pins.set_swdio_tms(true);
        self.jtag_cycle(); // Select-DR-Scan
        self.pins.set_swdio_tms(false);
        self.jtag_cycle(); // Capture-DR
        self.jtag_cycle(); // Shift-DR

        for _ in 0..index {
            self.jtag_cycle();
        }

        let bit = self.jtag_cycle_tdio(false);
        ack = (bit as u8) << 1;
        let bit = self.jtag_cycle_tdio(request & 0x04 != 0);
        ack |= bit as u8;
        let bit = self.jtag_cycle_tdio(request & 0x08 != 0);
        ack |= (bit as u8) << 2;

        let status = jtag_ack_status(ack);
        if status != TransferStatus::Ok {
            self.pins.set_swdio_tms(true);
            self.jtag_cycle(); // Exit1-DR
            self.jtag_transfer_exit();
            return status;
        }

        for _ in 0..31 {
            self.jtag_cycle_tdi(write_data & 1 != 0);
            write_data >>= 1;
        }

        let after = device_count.saturating_sub(index + 1);
        if after != 0 {
            self.jtag_cycle_tdi(write_data & 1 != 0);
            for _ in 1..after {
                self.jtag_cycle();
            }
            self.pins.set_swdio_tms(true);
            self.jtag_cycle(); // Bypass and Exit1-DR
        } else {
            self.pins.set_swdio_tms(true);
            self.jtag_cycle_tdi(write_data & 1 != 0);
        }

        self.jtag_transfer_exit();
        TransferStatus::Ok
    }

    pub fn jtag_write_abort(&mut self, index: usize, device_count: usize, mut data: u32) {
        self.pins.swdio_output();
        self.pins.set_swdio_tms(true);
        self.jtag_cycle(); // Select-DR-Scan
        self.pins.set_swdio_tms(false);
        self.jtag_cycle(); // Capture-DR
        self.jtag_cycle(); // Shift-DR

        for _ in 0..index {
            self.jtag_cycle();
        }

        self.pins.set_tdi(false);
        self.jtag_cycle(); // RnW = 0
        self.jtag_cycle(); // A2 = 0
        self.jtag_cycle(); // A3 = 0

        for _ in 0..31 {
            self.jtag_cycle_tdi(data & 1 != 0);
            data >>= 1;
        }

        let after = device_count.saturating_sub(index + 1);
        if after != 0 {
            self.jtag_cycle_tdi(data & 1 != 0);
            for _ in 1..after {
                self.jtag_cycle();
            }
            self.pins.set_swdio_tms(true);
            self.jtag_cycle(); // Bypass and Exit1-DR
        } else {
            self.pins.set_swdio_tms(true);
            self.jtag_cycle_tdi(data & 1 != 0);
        }

        self.jtag_transfer_exit();
    }

    pub fn set_pins(&mut self, values: u8, select: u8) -> u8 {
        if select & 0x01 != 0 {
            self.pins.set_swclk_tck(values & 0x01 != 0);
        }
        if select & 0x02 != 0 {
            self.pins.swdio_output();
            self.pins.set_swdio_tms(values & 0x02 != 0);
        }
        if select & 0x04 != 0 {
            self.pins.set_tdi(values & 0x04 != 0);
        }
        if select & 0x80 != 0 {
            self.pins.set_reset(values & 0x80 != 0);
        }
        self.pins.current_pin_state()
    }

    pub fn pin_state(&self) -> u8 {
        self.pins.current_pin_state()
    }

    #[allow(dead_code)]
    pub fn reset_target(&mut self) {
        self.pins.set_reset(false);
        time::delay_millis(20);
        self.pins.set_reset(true);
        time::delay_millis(20);
    }

    #[inline(never)]
    #[unsafe(link_section = ".fast")]
    pub fn swd_transfer(&mut self, request: u8, write_data: u32) -> TransferResult {
        if let Some(result) = self.pins.swd_transfer(
            request,
            write_data,
            self.turnaround_cycles,
            self.idle_cycles,
            self.data_phase_on_wait_fault,
        ) {
            return result;
        }

        let rn_w = request & 0x02 != 0;
        let swd_request = make_swd_request(request);
        if ENABLE_SWD_TRACE {
            trace_inc(0);
            trace_write(1, request as u32);
            trace_write(2, swd_request as u32);
            trace_write(7, self.pins.current_pin_state() as u32);
        }

        self.pins.swdio_output();
        self.write_bits(swd_request as u32, 8);

        self.pins.swdio_input();
        self.turnaround();
        let ack = self.read_bits(3) as u8;
        if ENABLE_SWD_TRACE {
            trace_write(3, ack as u32);
        }

        match ack {
            0b001 => {
                if rn_w {
                    let data = self.read_bits(32);
                    let parity = self.read_bit();
                    let expected = odd_parity(data);
                    if ENABLE_SWD_TRACE {
                        trace_write(5, data);
                        trace_write(6, parity as u32);
                    }
                    self.turnaround();
                    self.pins.swdio_output();
                    self.idle();
                    if parity == expected {
                        if ENABLE_SWD_TRACE {
                            trace_write(4, 1);
                        }
                        TransferResult {
                            status: TransferStatus::Ok,
                            data,
                        }
                    } else {
                        if ENABLE_SWD_TRACE {
                            trace_write(4, 0xbad);
                        }
                        TransferResult {
                            status: TransferStatus::ParityError,
                            data,
                        }
                    }
                } else {
                    self.turnaround();
                    self.pins.swdio_output();
                    self.write_bits(write_data, 32);
                    self.write_bit(odd_parity(write_data));
                    self.idle();
                    if ENABLE_SWD_TRACE {
                        trace_write(4, 1);
                    }
                    TransferResult {
                        status: TransferStatus::Ok,
                        data: 0,
                    }
                }
            }
            0b010 => {
                self.handle_wait_fault_data_phase(rn_w);
                self.turnaround();
                self.pins.swdio_output();
                self.idle();
                if ENABLE_SWD_TRACE {
                    trace_write(4, 2);
                }
                TransferResult {
                    status: TransferStatus::Wait,
                    data: 0,
                }
            }
            0b100 => {
                self.handle_wait_fault_data_phase(rn_w);
                self.turnaround();
                self.pins.swdio_output();
                self.idle();
                if ENABLE_SWD_TRACE {
                    trace_write(4, 4);
                }
                TransferResult {
                    status: TransferStatus::Fault,
                    data: 0,
                }
            }
            0b111 => {
                self.finish_protocol_error();
                if ENABLE_SWD_TRACE {
                    trace_write(4, 7);
                }
                TransferResult {
                    status: TransferStatus::NoAck,
                    data: 0,
                }
            }
            _ => {
                self.finish_protocol_error();
                if ENABLE_SWD_TRACE {
                    trace_write(4, 0x100 | ack as u32);
                }
                TransferResult {
                    status: TransferStatus::ProtocolError,
                    data: 0,
                }
            }
        }
    }

    #[inline(never)]
    #[unsafe(link_section = ".fast")]
    pub fn swd_write_block(
        &mut self,
        request: u8,
        data: &[u8],
        count: usize,
        wait_retries: usize,
    ) -> WriteBlockResult {
        if let Some(result) = self.pins.swd_write_block(
            request,
            data,
            count,
            wait_retries,
            self.turnaround_cycles,
            self.idle_cycles,
            self.data_phase_on_wait_fault,
        ) {
            return result;
        }

        let swd_request = make_swd_request(request) as u32;
        let mut done = 0;
        let mut input = 0;

        self.pins.swdio_output();

        while done < count {
            let write_data = u32::from_le_bytes([
                data[input],
                data[input + 1],
                data[input + 2],
                data[input + 3],
            ]);

            let mut retry = wait_retries;
            loop {
                let status = self.swd_write_transfer_precomputed(swd_request, write_data);
                if status != TransferStatus::Wait || retry == 0 {
                    if status != TransferStatus::Ok {
                        return WriteBlockResult { status, done };
                    }
                    break;
                }
                retry -= 1;
            }

            input += 4;
            done += 1;
        }

        WriteBlockResult {
            status: TransferStatus::Ok,
            done,
        }
    }

    fn connect_swd(&mut self) {
        const LINE_RESET: [u8; 8] = [0xff; 8];
        const JTAG_TO_SWD: [u8; 2] = [0x9e, 0xe7];
        const IDLE: [u8; 1] = [0x00];

        self.swj_sequence(64, &LINE_RESET);
        self.swj_sequence(16, &JTAG_TO_SWD);
        self.swj_sequence(64, &LINE_RESET);
        self.swj_sequence(8, &IDLE);
    }

    fn connect_jtag(&mut self) {
        self.pins.swdio_output();
        self.pins.set_swdio_tms(true);
        self.pins.set_tdi(true);
        for _ in 0..8 {
            self.pins.tck_cycle();
        }
        self.pins.set_swdio_tms(false);
        self.pins.tck_cycle();
    }

    #[inline(always)]
    fn write_bit(&mut self, bit: bool) {
        self.pins.swd_write_bit_cycle(bit);
    }

    #[inline(always)]
    fn write_bits(&mut self, bits: u32, count: usize) {
        self.pins.swd_write_bits(bits, count);
    }

    #[inline(always)]
    fn read_bit(&mut self) -> bool {
        self.pins.swclk_sample_swdio()
    }

    #[inline(always)]
    fn read_bits(&mut self, count: usize) -> u32 {
        self.pins.swclk_sample_swdio_bits(count)
    }

    #[inline(always)]
    fn turnaround(&mut self) {
        for _ in 0..self.turnaround_cycles {
            self.pins.swclk_cycle();
        }
    }

    fn handle_wait_fault_data_phase(&mut self, rn_w: bool) {
        if !self.data_phase_on_wait_fault {
            return;
        }

        if rn_w {
            let _ = self.read_bits(33);
        } else {
            self.turnaround();
            self.pins.swdio_output();
            self.write_bits(0, 33);
            self.pins.swdio_input();
        }
    }

    fn finish_protocol_error(&mut self) {
        self.pins.swdio_input();
        let _ = self.read_bits(self.turnaround_cycles as usize + 33);
        self.pins.swdio_output();
        self.idle();
    }

    #[inline(always)]
    fn idle(&mut self) {
        if self.idle_cycles != 0 {
            self.pins.set_swdio_tms(false);
            for _ in 0..self.idle_cycles {
                self.pins.swclk_cycle();
            }
        }
        self.pins.set_swdio_tms(true);
    }

    #[inline(always)]
    fn swd_write_transfer_precomputed(
        &mut self,
        swd_request: u32,
        write_data: u32,
    ) -> TransferStatus {
        if let Some(result) = self.pins.swd_transfer_precomputed(
            swd_request as u8,
            write_data,
            self.turnaround_cycles,
            self.idle_cycles,
            self.data_phase_on_wait_fault,
        ) {
            return result.status;
        }

        self.write_bits(swd_request, 8);

        self.pins.swdio_input();
        self.turnaround();
        let ack = self.read_bits(3) as u8;

        match ack {
            0b001 => {
                self.turnaround();
                self.pins.swdio_output();
                self.pins.swd_write_data_bits(write_data, 32);
                self.pins
                    .swd_write_data_bits(odd_parity(write_data) as u32, 1);
                self.idle();
                TransferStatus::Ok
            }
            0b010 => {
                self.handle_wait_fault_data_phase(false);
                self.turnaround();
                self.pins.swdio_output();
                self.idle();
                TransferStatus::Wait
            }
            0b100 => {
                self.handle_wait_fault_data_phase(false);
                self.turnaround();
                self.pins.swdio_output();
                self.idle();
                TransferStatus::Fault
            }
            0b111 => {
                self.finish_protocol_error();
                TransferStatus::NoAck
            }
            _ => {
                self.finish_protocol_error();
                TransferStatus::ProtocolError
            }
        }
    }

    #[inline(always)]
    fn jtag_cycle(&mut self) {
        self.pins.tck_cycle();
    }

    #[inline(always)]
    fn jtag_cycle_tdi(&mut self, bit: bool) {
        if self.pins.jtag_cycle_tdi(bit) {
            return;
        }
        self.pins.set_tdi(bit);
        self.pins.tck_cycle();
    }

    #[inline(always)]
    fn jtag_cycle_tdo(&mut self) -> bool {
        self.pins.tck_sample_tdo()
    }

    #[inline(always)]
    fn jtag_cycle_tdio(&mut self, tdi: bool) -> bool {
        if let Some(tdo) = self.pins.jtag_cycle_tdio(tdi) {
            return tdo;
        }
        self.pins.set_tdi(tdi);
        self.pins.tck_sample_tdo()
    }

    #[inline(always)]
    fn jtag_transfer_exit(&mut self) {
        self.jtag_cycle(); // Update-DR
        self.pins.set_swdio_tms(false);
        self.jtag_cycle(); // Run-Test/Idle
        self.pins.set_tdi(true);

        for _ in 0..self.idle_cycles {
            self.jtag_cycle();
        }
    }
}

#[inline(always)]
fn make_swd_request(dap_request: u8) -> u8 {
    let ap = dap_request & 0x01 != 0;
    let read = dap_request & 0x02 != 0;
    let a2 = dap_request & 0x04 != 0;
    let a3 = dap_request & 0x08 != 0;
    let parity = (ap as u8 ^ read as u8 ^ a2 as u8 ^ a3 as u8) & 1 != 0;

    1 | ((ap as u8) << 1)
        | ((read as u8) << 2)
        | ((a2 as u8) << 3)
        | ((a3 as u8) << 4)
        | ((parity as u8) << 5)
        | (1 << 7)
}

#[inline(always)]
fn odd_parity(value: u32) -> bool {
    let mut bits = value;
    bits ^= bits >> 16;
    bits ^= bits >> 8;
    bits ^= bits >> 4;
    (0x6996 >> (bits & 0x0f)) & 1 != 0
}

#[inline(always)]
fn jtag_ack_status(ack: u8) -> TransferStatus {
    match ack {
        0b001 => TransferStatus::Ok,
        0b010 => TransferStatus::Wait,
        0b100 => TransferStatus::Fault,
        0b111 => TransferStatus::NoAck,
        _ => TransferStatus::ProtocolError,
    }
}

#[inline(always)]
fn trace_write(index: usize, value: u32) {
    if ENABLE_SWD_TRACE && index < 16 {
        unsafe {
            let base = (&raw mut YBLINK_SWD_TRACE) as *mut [u32; 16] as *mut u32;
            core::ptr::write_volatile(base.add(index), value);
        }
    }
}

#[inline(always)]
fn trace_inc(index: usize) {
    if ENABLE_SWD_TRACE && index < 16 {
        unsafe {
            let base = (&raw mut YBLINK_SWD_TRACE) as *mut [u32; 16] as *mut u32;
            let value = core::ptr::read_volatile(base.add(index));
            core::ptr::write_volatile(base.add(index), value.wrapping_add(1));
        }
    }
}
