use crate::bus::Bus;

pub struct CPU {
    bus: Bus,

    // D0-D7
    // General purpose registers.
    d: [u32; 8],

    // A0-A7
    // General purpose registers. A7 is the stack pointer. In user mode, SP
    // refers to the User Stack Pointer (USP) register. During interrupts, the
    // active stack pointer SP is another register called the Interrupt Stack
    // Pointer or the System Stack Pointer.
    a: [u32; 8],

    // Condition code register. The lower 8 bits are available in user mode. The
    // upper 8 bits are only writeable in supervisor mode, but are available for
    // reading in user mode.
    //
    //  15 - 8   |   7 - 0
    // T.S. .III | ...X NZVC
    //
    // T  trace mode
    // S  supervisor state (if clear, the stack pointer points to the USP,
    //                      otherwise it's the ISP)
    // I  IRQ mask
    // X  Extend
    // N  Negative/sign
    // Z  Zero
    // V  Overflow
    // C  Carry
    ccr: u16,

    // Program counter. Despite being 32-bits wide, only the lowest 24-bits
    // output to any pins in the 68000, resulting in a 16MB address space.
    pc: u32,
}

impl CPU {
    pub fn new_68k() -> Self {
        Self {
            bus: Bus::new_memory_bus(),

            d: [0; 8],
            a: [0; 8],
            ccr: 0,
            pc: 0,
        }
    }

    fn set_supervisor_state(&mut self, v: bool) {
        if v {
            self.ccr |= 0b0010_0000;
            // TODO set stack pointer to ISP?
        } else {
            self.ccr &= !0b0010_0000;
            // TODO set stack pointer to USP?
        }
    }

    fn set_supervisor_trace(&mut self, v: bool) {
        if v {
            self.ccr |= 0b1000_0000;
        } else {
            self.ccr &= !0b1000_0000;
        }
    }

    fn set_extend(&mut self, v: bool) {
        if v {
            self.ccr |= 0b0001_0000;
        } else {
            self.ccr &= !0b0001_0000;
        }
    }

    fn set_sign(&mut self, v: bool) {
        if v {
            self.ccr |= 0b0000_1000;
        } else {
            self.ccr &= !0b0000_1000;
        }
    }

    fn set_zero(&mut self, v: bool) {
        if v {
            self.ccr |= 0b0000_0100;
        } else {
            self.ccr &= !0b0000_0100;
        }
    }

    fn set_overflow(&mut self, v: bool) {
        if v {
            self.ccr |= 0b0000_0010;
        } else {
            self.ccr &= !0b0000_0010;
        }
    }

    fn set_carry(&mut self, v: bool) {
        if v {
            self.ccr |= 0b0000_0001;
        } else {
            self.ccr &= !0b0000_0001;
        }
    }

    fn set_flags(&mut self, v: u8) {
        self.ccr = (self.ccr & 0xff00) | ((v as u16) & 0b001_1111);
    }

    pub fn step(&mut self) -> u64 {
        0
    }
}
