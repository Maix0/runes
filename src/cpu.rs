pub struct Cpu {
    pub flags: Flags,
    pub registers: Registers,
    pub remaining_cycles: u8,
    pub addr: u16,
    pub rel: u16,
    pub fetch: u8,
    pub immediate: bool,
}

#[allow(unused_variables)]
impl Cpu {
    pub fn cycle_one(&mut self, bus: &mut crate::bus::CpuBus) {
        if self.remaining_cycles == 0 {
            let opcode = bus.read(self.registers.pc);
            match instructions::Instructions::try_from(opcode) {
                Ok(instruction) => {
                    self.remaining_cycles = match instruction.cycles() {
                        instructions::Cycles::Normal(n) => n,
                        instructions::Cycles::Page(n) => n,
                        instructions::Cycles::Branch(n) => n,
                    };
                    let mut memory = [0u8; 2];

                    for i in 1..instruction.bytes() {
                        memory[i as usize] = bus.read(self.registers.pc + i as u16);
                    }
                    self.registers.pc += instruction.bytes() as u16;
                    instruction.run(memory, self, bus);
                }
                Err(_) => {
                    eprintln!(
                        "Illegal Instruction `{opcode:20X}` at byte `{}`",
                        self.registers.pc
                    );
                    self.registers.pc += 1;
                }
            };
        }
    }

    pub fn fetch(&mut self, bus: &mut crate::bus::CpuBus) -> u8 {
        if !self.immediate {
            self.fetch = bus.read(self.addr);
        }
        self.immediate = false;
        self.fetch
    }

    pub fn accumulator(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        self.fetch = self.registers.a;
    }

    pub fn immediate(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        self.fetch = u8::from_be(input[0]);
        self.immediate = true;
    }

    pub fn absolute(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        let input = u16::from_be_bytes(input);
        self.addr = input;
    }

    pub fn zero_page(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        let input = u8::from_be(input[0]);
        self.addr = input as u16;
    }

    pub fn zero_page_x(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        let input = u8::from_be(input[0]).wrapping_add(self.registers.x);
        self.addr = input as u16;
    }

    pub fn zero_page_y(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        let input = u8::from_be(input[0]).wrapping_add(self.registers.y);
        self.addr = input as u16;
    }

    pub fn absolute_x(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        let input_ptr = u16::from_be_bytes(input).wrapping_add(self.registers.x as u16);
        if (input_ptr & 0xFF00) != ((input[1] as u16) << 8) {
            self.remaining_cycles += 1;
        }

        self.addr = input_ptr as u16;
    }

    pub fn absolute_y(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        let input_ptr = u16::from_be_bytes(input).wrapping_add(self.registers.y as u16);
        if (input_ptr & 0xFF00) != ((input[1] as u16) << 8) {
            self.remaining_cycles += 1;
        }

        self.addr = input_ptr;
    }

    pub fn implied(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        self.fetch = self.registers.a;
    }

    pub fn relative(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        let input = (i8::from_be_bytes([input[0]]) as i32)
            .wrapping_add(self.registers.pc as i32)
            .clamp(0, u16::MAX as i32) as u16;
        self.addr = input as u16;
    }

    pub fn indirect_x(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        let input = {
            ((bus.read(input[0] as u16 + 1 + self.registers.x as u16) as u16) << 8)
                | (bus.read(input[0] as u16 + self.registers.x as u16) as u16 as u16)
        };

        self.addr = input as u16;
    }

    pub fn indirect_y(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        let input_ptr = u16::from_be_bytes(input).wrapping_add(self.registers.y as u16);
        if (input_ptr & 0xFF00) != ((input[1] as u16) << 8) {
            self.remaining_cycles += 1;
        }

        self.addr = input_ptr;
    }

    pub fn indirect(&mut self, input: [u8; 2], bus: &mut crate::bus::CpuBus) {
        let input = ((bus.read(if input[0] == 0x00FF {
            (input[1] as u16) << 8
        } else {
            input[1] as u16
        }) as u16)
            << 8)
            | (bus.read(input[0] as u16) as u16);
        self.addr = input;
    }
}

#[derive(Debug, Clone)]
pub struct Registers {
    pub stack_pointer: u8,
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
}

bitflags::bitflags! {
    pub struct Flags: u8 {
        const CARRY =               1 << 0;
        const ZERO =                1 << 1;
        const INTERRUPT_DISABLE =   1 << 2;
        const DECIMAL =             1 << 3; // unused
        const BREAK =               1 << 4;
        const UNUSED =              1 << 5; // unused
        const OVERFLOW =            1 << 6;
        const NEGATIVE =            1 << 7;

        const C = Self::CARRY.bits;
        const Z = Self::ZERO.bits;
        const I = Self::INTERRUPT_DISABLE.bits;
        const D = Self::DECIMAL.bits;
        const B = Self::BREAK.bits;
        const U = Self::UNUSED.bits;
        const V = Self::OVERFLOW.bits;
        const N = Self::NEGATIVE.bits;

    }
}

#[allow(unused)]
#[allow(dead_code)]
pub mod instructions {
    use super::*;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/out/instructions.rs"));
}
