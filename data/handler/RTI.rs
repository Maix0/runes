{
    let status_bits = bus.read(0x100 + (cpu.registers.stack_pointer as u16));
    cpu.flags = Flags::from_bits_truncate(status_bits) & (!Flags::B) & (!Flags::U);

    cpu.registers.stack_pointer += 1;
    let low = bus.read(0x100 + (cpu.registers.stack_pointer as u16));
    cpu.registers.stack_pointer += 1;
    let high = bus.read(0x100 + (cpu.registers.stack_pointer as u16));
    cpu.registers.stack_pointer += 1;

    cpu.registers.pc = u16::from_be_bytes([high, low]);
    0
}
