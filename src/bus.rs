pub struct CpuBus {
    memory: Box<[u8; Self::BUS_SIZE]>,
}

impl CpuBus {
    const BUS_SIZE: usize = 0xFFFF;
    pub fn new() -> Self {
        Self {
            memory: unsafe {
                vec![0; Self::BUS_SIZE]
                    .into_boxed_slice()
                    .try_into()
                    .unwrap_unchecked()
            },
        }
    }
    pub fn write(&mut self, byte: u8, address: u16) {
        self.memory[address as usize] = byte;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    /*
    pub fn write(&mut self, byte: u8, address: u16) {
        let addr_usize: usize = address.into();
        match address {
            0x0000..=0x07FF => {
                self.memory[addr_usize] = byte;
            }
            0x0800..=0x0FFF => {
                self.memory[addr_usize - 0x0800] = byte;
            }
            0x1000..=0x17FF => {
                self.memory[addr_usize - 0x1000] = byte;
            }
            0x1800..=0x1FFF => {
                self.memory[addr_usize - 0x1800] = byte;
            }
            0x2000..=0x2007 => {
                self.memory[addr_usize] = byte;
            }
            0x2008..=0x3FFF => {
                self.memory[addr_usize] = byte;
            }
            0x4000..=0x4017 => {
                self.memory[addr_usize] = byte;
            }
            0x4018..=0x401F => {
                self.memory[addr_usize] = byte;
            }
            0x4020..=0xFFFF => {
                self.memory[addr_usize] = byte;
            }
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let addr_usize: usize = match address {
            0x0000..=0x07FF => address,
            0x0800..=0x0FFF => address - 0x0800,
            0x1000..=0x17FF => address - 0x1000,
            0x1800..=0x1FFF => address - 0x1800,
            0x2000..=0x2007 => address,
            0x2008..=0x3FFF => address,
            0x4000..=0x4017 => address,
            0x4018..=0x401F => address,
            0x4020..=0xFFFF => address,
        }
        .into();

        self.memory[addr_usize]
    }
    */
}

pub struct PpuBus {
    memory: Box<[u8; Self::BUS_SIZE]>,
}

impl PpuBus {
    const BUS_SIZE: usize = 0x3FFF;
    pub fn new() -> Self {
        Self {
            memory: unsafe {
                vec![0; Self::BUS_SIZE]
                    .into_boxed_slice()
                    .try_into()
                    .unwrap_unchecked()
            },
        }
    }
}
